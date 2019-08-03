#![feature(async_await)]
use futures::{ StreamExt, TryStreamExt};
use futures::future::join;
use h2_support::prelude::*;

#[tokio::test]
async fn recv_push_works() {
    let _ = env_logger::try_init();

    let (io, mut srv) = mock::new();
    let mock = async {
        let settings = srv.assert_client_handshake().await;
        assert_default_settings!(settings);
        srv.recv_frame(
            frames::headers(1)
                .request("GET", "https://http2.akamai.com/")
                .eos(),
        )
        .await;
        srv.send_frame(frames::headers(1).response(404)).await;
        srv.send_frame(
            frames::push_promise(1, 2).request("GET", "https://http2.akamai.com/style.css"),
        )
        .await;
        srv.send_frame(frames::data(1, "").eos()).await;
        srv.send_frame(frames::headers(2).response(200)).await;
        srv.send_frame(frames::data(2, "promised_data").eos()).await;
    };
    let h2 = async {
        let (mut client, mut h2) = client::handshake(io).await.unwrap();
        let request = Request::builder()
            .method(Method::GET)
            .uri("https://http2.akamai.com/")
            .body(())
            .unwrap();
        let (mut resp, _) = client.send_request(request, true).unwrap();
        let pushed = resp.push_promises();
        let check_resp_status = async {
            let resp = resp.await.unwrap();
            assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        };
        let check_pushed_response = async {
            let p = pushed.and_then(|headers| {
                async {
                    let (request, response) = headers.into_parts();
                    assert_eq!(request.into_parts().0.method, Method::GET);
                    let resp = response.await.unwrap();
                    assert_eq!(resp.status(), StatusCode::OK);
                    let b = resp.into_body().try_concat().await.unwrap();
                    assert_eq!(b, "promised_data");
                    Ok(())
                }
            });
            let ps: Vec<_> = p.collect().await;
            assert_eq!(1, ps.len())
        };

        h2.drive(join(check_resp_status, check_pushed_response))
            .await;
    };

    join(h2, mock).await;
}

#[tokio::test]
async fn pushed_streams_arent_dropped_too_early() {
    // tests that by default, received push promises work
    let _ = env_logger::try_init();

    let (io, mut srv) = mock::new();
    let mock = async {
        let settings = srv.assert_client_handshake().await;
        assert_default_settings!(settings);
        srv.recv_frame(
            frames::headers(1)
                .request("GET", "https://http2.akamai.com/")
                .eos(),
        )
        .await;
        srv.send_frame(frames::headers(1).response(404)).await;
        srv.send_frame(
            frames::push_promise(1, 2).request("GET", "https://http2.akamai.com/style.css"),
        )
        .await;
        srv.send_frame(
            frames::push_promise(1, 4).request("GET", "https://http2.akamai.com/style2.css"),
        )
        .await;
        srv.send_frame(frames::data(1, "").eos()).await;
        idle_ms(10).await;
        srv.send_frame(frames::headers(2).response(200)).await;
        srv.send_frame(frames::headers(4).response(200).eos()).await;
        srv.send_frame(frames::data(2, "").eos()).await;
        srv.recv_frame(frames::go_away(4)).await;
    };

    let h2 = async {
        let (mut client, mut h2) = client::handshake(io).await.unwrap();
        let request = Request::builder()
            .method(Method::GET)
            .uri("https://http2.akamai.com/")
            .body(())
            .unwrap();
        let (mut resp, _) = client.send_request(request, true).unwrap();
        let pushed = resp.push_promises();
        let check_status = async {
            let resp = resp.await.unwrap();
            assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        };

        let check_pushed = async {
            let p = pushed.and_then(|headers| {
                async {
                    let (request, response) = headers.into_parts();
                    assert_eq!(request.into_parts().0.method, Method::GET);
                    let resp = response.await.unwrap();
                    assert_eq!(resp.status(), StatusCode::OK);
                    Ok(())
                }
            });
            let ps: Vec<_> = p.collect().await;
            assert_eq!(2, ps.len())
        };

        h2.drive(join(check_status, check_pushed)).await;
        h2.await.expect("client");
    };

    join(h2, mock).await;
}

#[tokio::test]
async fn recv_push_when_push_disabled_is_conn_error() {
    let _ = env_logger::try_init();

    let (io, mut srv) = mock::new();
    let mock = async {
        let _ = srv.assert_client_handshake().await;
        srv.recv_frame(
            frames::headers(1)
                .request("GET", "https://http2.akamai.com/")
                .eos(),
        )
        .await;
        srv.send_frame(
            frames::push_promise(1, 3).request("GET", "https://http2.akamai.com/style.css"),
        )
        .await;
        srv.send_frame(frames::headers(1).response(200).eos()).await;
        srv.recv_frame(frames::go_away(0).protocol_error()).await;
    };

    let h2 = async {
        let (mut client, h2) = client::Builder::new()
            .enable_push(false)
            .handshake::<_, Bytes>(io)
            .await
            .unwrap();
        let request = Request::builder()
            .method(Method::GET)
            .uri("https://http2.akamai.com/")
            .body(())
            .unwrap();

        let req = async {
            let res = client.send_request(request, true).unwrap().0.await;
            let err = res.unwrap_err();
            assert_eq!(
                err.to_string(),
                "protocol error: unspecific protocol error detected"
            );
        };

        // client should see a protocol error
        let conn = async {
            let res = h2.await;
            let err = res.unwrap_err();
            assert_eq!(
                err.to_string(),
                "protocol error: unspecific protocol error detected"
            );
        };

        join(conn, req).await;
    };

    join(h2, mock).await;
}

#[tokio::test]
async fn pending_push_promises_reset_when_dropped() {
    let _ = env_logger::try_init();

    let (io, mut srv) = mock::new();
    let srv = async {
        let settings = srv.assert_client_handshake().await;
        assert_default_settings!(settings);
        srv.recv_frame(
            frames::headers(1)
                .request("GET", "https://http2.akamai.com/")
                .eos(),
        )
        .await;
        srv.send_frame(
            frames::push_promise(1, 2).request("GET", "https://http2.akamai.com/style.css"),
        )
        .await;
        srv.send_frame(frames::headers(1).response(200).eos()).await;
        srv.recv_frame(frames::reset(2).cancel()).await;
    };

    let client = async {
        let (mut client, mut conn) = client::handshake(io).await.unwrap();
        let request = Request::builder()
            .method(Method::GET)
            .uri("https://http2.akamai.com/")
            .body(())
            .unwrap();
        let req = async {
            let resp = client
                .send_request(request, true)
                .unwrap()
                .0
                .await
                .expect("response");
            assert_eq!(resp.status(), StatusCode::OK);
        };

        let _ = conn.drive(req).await;
        conn.await.expect("client");
        drop(client);
    };

    join(client, srv).await;
}

#[tokio::test]
async fn recv_push_promise_over_max_header_list_size() {
    let _ = env_logger::try_init();
    let (io, mut srv) = mock::new();

    let srv = async {
        let settings = srv.assert_client_handshake().await;
        assert_frame_eq(settings, frames::settings().max_header_list_size(10));
        srv.recv_frame(
            frames::headers(1)
                .request("GET", "https://http2.akamai.com/")
                .eos(),
        )
        .await;
        srv.send_frame(
            frames::push_promise(1, 2).request("GET", "https://http2.akamai.com/style.css"),
        )
        .await;
        srv.recv_frame(frames::reset(2).refused()).await;
        srv.send_frame(frames::headers(1).response(200).eos()).await;
        idle_ms(10).await;
    };

    let client = async {
        let (mut client, mut conn) = client::Builder::new()
            .max_header_list_size(10)
            .handshake::<_, Bytes>(io)
            .await
            .expect("handshake");
        let request = Request::builder()
            .uri("https://http2.akamai.com/")
            .body(())
            .unwrap();

        let req = async {
            let err = client
                .send_request(request, true)
                .expect("send_request")
                .0
                .await
                .expect_err("response");
            assert_eq!(err.reason(), Some(Reason::REFUSED_STREAM));
        };

        conn.drive(req).await;
        conn.await.expect("client");
    };
    join(client, srv).await;
}

#[tokio::test]
async fn recv_invalid_push_promise_headers_is_stream_protocol_error() {
    // Unsafe method or content length is stream protocol error
    let _ = env_logger::try_init();

    let (io, mut srv) = mock::new();
    let mock = async {
        let settings = srv.assert_client_handshake().await;
        assert_default_settings!(settings);
        srv.recv_frame(
            frames::headers(1)
                .request("GET", "https://http2.akamai.com/")
                .eos(),
        )
        .await;
        srv.send_frame(frames::headers(1).response(404)).await;
        srv.send_frame(
            frames::push_promise(1, 2).request("POST", "https://http2.akamai.com/style.css"),
        )
        .await;
        srv.send_frame(
            frames::push_promise(1, 4)
                .request("GET", "https://http2.akamai.com/style.css")
                .field(http::header::CONTENT_LENGTH, 1),
        )
        .await;
        srv.send_frame(
            frames::push_promise(1, 6)
                .request("GET", "https://http2.akamai.com/style.css")
                .field(http::header::CONTENT_LENGTH, 0),
        )
        .await;
        srv.send_frame(frames::headers(1).response(404).eos()).await;
        srv.recv_frame(frames::reset(2).protocol_error()).await;
        srv.recv_frame(frames::reset(4).protocol_error()).await;
        srv.send_frame(frames::headers(6).response(200).eos()).await;
    };

    let h2 = async {
        let (mut client, mut h2) = client::handshake(io).await.unwrap();
        let request = Request::builder()
            .method(Method::GET)
            .uri("https://http2.akamai.com/")
            .body(())
            .unwrap();
        let (mut resp, _) = client.send_request(request, true).unwrap();
        let check_pushed_response = async {
            let pushed = resp.push_promises();
            let p = pushed.and_then(|headers| headers.into_parts().1);
            let ps: Vec<_> = p.collect().await;
            // CONTENT_LENGTH = 0 is ok
            assert_eq!(1, ps.len());
        };
        h2.drive(check_pushed_response).await;
    };

    join(h2, mock).await;
}

#[test]
#[ignore]
fn recv_push_promise_with_wrong_authority_is_stream_error() {
    // if server is foo.com, :authority = bar.com is stream error
}

#[tokio::test]
async fn recv_push_promise_skipped_stream_id() {
    let _ = env_logger::try_init();

    let (io, mut srv) = mock::new();
    let mock = async {
        let settings = srv.assert_client_handshake().await;
        assert_default_settings!(settings);
        srv.recv_frame(
            frames::headers(1)
                .request("GET", "https://http2.akamai.com/")
                .eos(),
        )
        .await;
        srv.send_frame(
            frames::push_promise(1, 4).request("GET", "https://http2.akamai.com/style.css"),
        )
        .await;
        srv.send_frame(
            frames::push_promise(1, 2).request("GET", "https://http2.akamai.com/style.css"),
        )
        .await;
        srv.recv_frame(frames::go_away(0).protocol_error()).await;
    };

    let h2 = async {
        let (mut client, h2) = client::handshake(io).await.unwrap();
        let request = Request::builder()
            .method(Method::GET)
            .uri("https://http2.akamai.com/")
            .body(())
            .unwrap();

        let req = async {
            let res = client.send_request(request, true).unwrap().0.await;
            assert!(res.is_err());
        };

        // client should see a protocol error
        let conn = async {
            let res = h2.await;
            let err = res.unwrap_err();
            assert_eq!(
                err.to_string(),
                "protocol error: unspecific protocol error detected"
            );
        };

        join(conn, req).await;
    };

    join(h2, mock).await;
}

#[tokio::test]
async fn recv_push_promise_dup_stream_id() {
    let _ = env_logger::try_init();

    let (io, mut srv) = mock::new();
    let mock = async {
        let settings = srv.assert_client_handshake().await;
        assert_default_settings!(settings);
        srv.recv_frame(
            frames::headers(1)
                .request("GET", "https://http2.akamai.com/")
                .eos(),
        )
        .await;
        srv.send_frame(
            frames::push_promise(1, 2).request("GET", "https://http2.akamai.com/style.css"),
        )
        .await;
        srv.send_frame(
            frames::push_promise(1, 2).request("GET", "https://http2.akamai.com/style.css"),
        )
        .await;
        srv.recv_frame(frames::go_away(0).protocol_error()).await;
    };

    let h2 = async {
        let (mut client, h2) = client::handshake(io).await.unwrap();
        let request = Request::builder()
            .method(Method::GET)
            .uri("https://http2.akamai.com/")
            .body(())
            .unwrap();

        let req = async {
            let res = client.send_request(request, true).unwrap().0.await;
            assert!(res.is_err());
        };

        // client should see a protocol error
        let conn = async {
            let res = h2.await;
            let err = res.unwrap_err();
            assert_eq!(
                err.to_string(),
                "protocol error: unspecific protocol error detected"
            );
        };

        join(conn, req).await;
    };

    join(h2, mock).await;
}
