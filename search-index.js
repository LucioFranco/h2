var N=null,E="",T="t",U="u",searchIndex={};
var R=["reason","option","Returns the error if the error is an io::Error","error","handshake","Indicates the initial window size (in octets) for…","pingpong","request","stream_id","Returns the stream ID of the response stream.","streamid","Handshake","Connection","set_target_window_size","Sets the target window size for the whole connection.","ping_pong","Takes a `PingPong` instance from the connection.","initial_window_size","initial_connection_window_size","max_frame_size","Indicates the size (in octets) of the largest HTTP/2.0…","max_header_list_size","Sets the max size of received header frames.","max_concurrent_streams","Sets the maximum number of concurrent streams.","max_concurrent_reset_streams","Sets the maximum number of concurrent locally reset streams.","reset_stream_duration","duration","sendstream","send_reset","poll_reset","Polls to be notified when the client resets this stream.","headermap","release_capacity","result","to_string","try_from","borrow_mut","try_into","type_id","to_owned","clone_into","equivalent","borrow","typeid","h2::client","into_future","h2::server","builder","formatter","description","SendRequest","Builder","ResponseFuture","PushPromise","PushedResponseFuture","SendResponse","SendStream","RecvStream","ReleaseCapacity","PingPong","StreamId","ReadySendRequest","PushPromises"];

searchIndex["h2"]={"doc":"An asynchronous, HTTP/2.0 server and client implementation.","i":[[3,"Error","h2","Represents HTTP/2.0 operation errors.",N,N],[3,"Reason",E,"HTTP/2.0 error codes.",N,N],[3,R[58],E,"Sends the body stream and trailers to the remote peer.",N,N],[3,R[62],E,"A stream identifier, as described in [Section 5.1.1] of…",N,N],[3,R[59],E,"Receives the body stream and trailers from the remote peer.",N,N],[3,R[60],E,"A handle to release window capacity to a remote stream.",N,N],[3,R[61],E,"A handle to send and receive PING frames with the peer.",N,N],[3,"Ping",E,"Sent via [`PingPong`][] to send a PING frame to a peer.",N,N],[3,"Pong",E,"Received via [`PingPong`][] when a peer acknowledges a…",N,N],[11,R[0],E,"If the error was caused by the remote peer, the error…",0,[[["self"]],[[R[0]],[R[1],[R[0]]]]]],[11,"is_io",E,"Returns the true if the error is an io::Error",0,[[["self"]],["bool"]]],[11,"get_io",E,R[2],0,[[["self"]],[[R[1],[R[3]]],[R[3]]]]],[11,"into_io",E,R[2],0,[[],[[R[3]],[R[1],[R[3]]]]]],[18,"NO_ERROR",E,"The associated condition is not a result of an error.",1,N],[18,"PROTOCOL_ERROR",E,"The endpoint detected an unspecific protocol error.",1,N],[18,"INTERNAL_ERROR",E,"The endpoint encountered an unexpected internal error.",1,N],[18,"FLOW_CONTROL_ERROR",E,"The endpoint detected that its peer violated the…",1,N],[18,"SETTINGS_TIMEOUT",E,"The endpoint sent a SETTINGS frame but did not receive a…",1,N],[18,"STREAM_CLOSED",E,"The endpoint received a frame after a stream was…",1,N],[18,"FRAME_SIZE_ERROR",E,"The endpoint received a frame with an invalid size.",1,N],[18,"REFUSED_STREAM",E,"The endpoint refused the stream prior to performing any…",1,N],[18,"CANCEL",E,"Used by the endpoint to indicate that the stream is no…",1,N],[18,"COMPRESSION_ERROR",E,"The endpoint is unable to maintain the header compression…",1,N],[18,"CONNECT_ERROR",E,"The connection established in response to a CONNECT…",1,N],[18,"ENHANCE_YOUR_CALM",E,"The endpoint detected that its peer is exhibiting a…",1,N],[18,"INADEQUATE_SECURITY",E,"The underlying transport has properties that do not meet…",1,N],[18,"HTTP_1_1_REQUIRED",E,"The endpoint requires that HTTP/1.1 be used instead of…",1,N],[11,R[51],E,"Get a string description of the error code.",1,[[["self"]],["str"]]],[0,"client",E,"Client implementation of the HTTP/2.0 protocol.",N,N],[3,R[11],R[46],"Performs the HTTP/2.0 connection handshake.",N,N],[3,R[52],E,"Initializes new HTTP/2.0 streams on a connection by…",N,N],[3,R[63],E,"Returns a `SendRequest` instance once it is ready to send…",N,N],[3,R[12],E,"Manages all state associated with an HTTP/2.0 client…",N,N],[3,R[54],E,"A future of an HTTP response.",N,N],[3,R[56],E,"A future of a pushed HTTP response.",N,N],[3,R[55],E,"A pushed response and corresponding request headers",N,N],[3,R[64],E,"A stream of pushed responses and corresponding promised…",N,N],[3,R[53],E,"Builds client connections with custom configuration values.",N,N],[5,R[4],E,"Creates a new configured HTTP/2.0 client with default…",N,[[[T]],[[R[4],["bytes"]],["bytes"]]]],[11,"poll_ready",E,"Returns `Ready` when the connection can initialize a new…",2,[[["self"]],[[R[3]],["poll",[R[3]]]]]],[11,"ready",E,"Consumes `self`, returning a future that returns `self`…",2,[[],["readysendrequest"]]],[11,"send_request",E,"Sends a HTTP/2.0 request to the server.",2,[[["self"],["bool"],[R[7]]],[[R[3]],[R[35],[R[3]]]]]],[11,"new",E,"Returns a new client builder instance initialized with…",3,[[],[R[49]]]],[11,R[17],E,R[5],3,[[["u32"],["self"]],["self"]]],[11,R[18],E,R[5],3,[[["u32"],["self"]],["self"]]],[11,R[19],E,R[20],3,[[["u32"],["self"]],["self"]]],[11,R[21],E,R[22],3,[[["u32"],["self"]],["self"]]],[11,R[23],E,R[24],3,[[["u32"],["self"]],["self"]]],[11,"initial_max_send_streams",E,"Sets the initial maximum of locally initiated (send)…",3,[[["self"],["usize"]],["self"]]],[11,R[25],E,R[26],3,[[["self"],["usize"]],["self"]]],[11,R[27],E,"Sets the duration to remember locally reset streams.",3,[[["self"],[R[28]]],["self"]]],[11,"enable_push",E,"Enables or disables server push promises.",3,[[["self"],["bool"]],["self"]]],[11,R[4],E,"Creates a new configured HTTP/2.0 client backed by `io`.",3,[[["self"],[T]],[R[4]]]],[11,R[13],E,R[14],4,[[["u32"],["self"]]]],[11,R[15],E,R[16],4,[[["self"]],[[R[6]],[R[1],[R[6]]]]]],[11,R[8],E,R[9],5,[[["self"]],[R[10]]]],[11,"push_promises",E,"Returns a stream of PushPromises",5,[[["self"]],["pushpromises"]]],[11,R[7],E,"Returns a reference to the push promise's request headers.",6,[[["self"]],[R[7]]]],[11,"request_mut",E,"Returns a mutable reference to the push promise's request…",6,[[["self"]],[R[7]]]],[11,"into_parts",E,"Consumes `self`, returning the push promise's request…",6,[[]]],[11,R[8],E,R[9],7,[[["self"]],[R[10]]]],[0,"server","h2","Server implementation of the HTTP/2.0 protocol.",N,N],[3,R[11],R[48],"In progress HTTP/2.0 connection handshake future.",N,N],[3,R[12],E,"Accepts inbound HTTP/2.0 streams on a connection.",N,N],[3,R[53],E,"Builds server connections with custom configuration values.",N,N],[3,R[57],E,"Send a response back to the client",N,N],[5,R[4],E,"Creates a new configured HTTP/2.0 server with default…",N,[[[T]],[["bytes"],[R[4],["bytes"]]]]],[11,R[13],E,R[14],8,[[["u32"],["self"]]]],[11,"poll_close",E,"Returns `Ready` when the underlying connection has closed.",8,[[["self"]],[[R[3]],["poll",[R[3]]]]]],[11,"abrupt_shutdown",E,"Sets the connection to a GOAWAY state.",8,[[["self"],[R[0]]]]],[11,"graceful_shutdown",E,"Starts a [graceful shutdown][1] process.",8,[[["self"]]]],[11,R[15],E,R[16],8,[[["self"]],[[R[6]],[R[1],[R[6]]]]]],[11,"new",E,"Returns a new server builder instance initialized with…",9,[[],[R[49]]]],[11,R[17],E,R[5],9,[[["u32"],["self"]],["self"]]],[11,R[18],E,R[5],9,[[["u32"],["self"]],["self"]]],[11,R[19],E,R[20],9,[[["u32"],["self"]],["self"]]],[11,R[21],E,R[22],9,[[["u32"],["self"]],["self"]]],[11,R[23],E,R[24],9,[[["u32"],["self"]],["self"]]],[11,R[25],E,R[26],9,[[["self"],["usize"]],["self"]]],[11,R[27],E,R[26],9,[[["self"],[R[28]]],["self"]]],[11,R[4],E,"Creates a new configured HTTP/2.0 server backed by `io`.",9,[[["self"],[T]],[R[4]]]],[11,"send_response",E,"Send a response to a client request.",10,[[["self"],["bool"],["response"]],[[R[3]],[R[29]],[R[35],[R[29],R[3]]]]]],[11,R[30],E,"Send a stream reset to the peer.",10,[[["self"],[R[0]]]]],[11,R[31],E,R[32],10,[[["self"]],[[R[0]],[R[3]],["poll",[R[0],R[3]]]]]],[11,R[8],E,R[9],10,[[["self"]],[R[10]]]],[11,"reserve_capacity","h2","Requests capacity to send data.",11,[[["self"],["usize"]]]],[11,"capacity",E,"Returns the stream's current send capacity.",11,[[["self"]],["usize"]]],[11,"poll_capacity",E,"Requests to be notified when the stream's capacity…",11,[[["self"]],[[R[3]],[R[1],["usize"]],["poll",[R[1],R[3]]]]]],[11,"send_data",E,"Sends a single data frame to the remote peer.",11,[[["self"],["bool"],["b"]],[[R[35],[R[3]]],[R[3]]]]],[11,"send_trailers",E,"Sends trailers to the remote peer.",11,[[["self"],[R[33]]],[[R[35],[R[3]]],[R[3]]]]],[11,R[30],E,"Resets the stream.",11,[[["self"],[R[0]]]]],[11,R[31],E,R[32],11,[[["self"]],[[R[0]],[R[3]],["poll",[R[0],R[3]]]]]],[11,R[8],E,"Returns the stream ID of this `SendStream`.",11,[[["self"]],[R[10]]]],[11,"is_end_stream",E,"Returns true if the receive half has reached the end of…",12,[[["self"]],["bool"]]],[11,R[34],E,"Get a mutable reference to this streams `ReleaseCapacity`.",12,[[["self"]],["releasecapacity"]]],[11,"poll_trailers",E,"Returns received trailers.",12,[[["self"]],[[R[1],[R[33]]],[R[3]],["poll",[R[1],R[3]]]]]],[11,R[8],E,"Returns the stream ID of this stream.",12,[[["self"]],[R[10]]]],[11,R[8],E,"Returns the stream ID of the stream whose capacity will be…",13,[[["self"]],[R[10]]]],[11,R[34],E,"Release window capacity back to remote stream.",13,[[["self"],["usize"]],[[R[35],[R[3]]],[R[3]]]]],[11,"send_ping",E,"Send a `PING` frame to the peer.",14,[[["self"],["ping"]],[[R[35],[R[3]]],[R[3]]]]],[11,"poll_pong",E,"Polls for the acknowledgement of a previously [sent][]…",14,[[["self"]],[[R[3]],["poll",["pong",R[3]]],["pong"]]]],[11,"opaque",E,"Creates a new opaque `Ping` to be sent via a [`PingPong`][].",15,[[],["ping"]]],[11,"from",E,E,0,[[[T]],[T]]],[11,"into",E,E,0,[[],[U]]],[11,R[36],E,E,0,[[["self"]],["string"]]],[11,R[37],E,E,0,[[[U]],[R[35]]]],[11,R[44],E,E,0,[[["self"]],[T]]],[11,R[40],E,E,0,[[["self"]],[R[45]]]],[11,R[38],E,E,0,[[["self"]],[T]]],[11,R[39],E,E,0,[[],[R[35]]]],[11,"from",E,E,1,[[[T]],[T]]],[11,"into",E,E,1,[[],[U]]],[11,R[41],E,E,1,[[["self"]],[T]]],[11,R[42],E,E,1,[[[T],["self"]]]],[11,R[36],E,E,1,[[["self"]],["string"]]],[11,R[37],E,E,1,[[[U]],[R[35]]]],[11,R[44],E,E,1,[[["self"]],[T]]],[11,R[40],E,E,1,[[["self"]],[R[45]]]],[11,R[38],E,E,1,[[["self"]],[T]]],[11,R[39],E,E,1,[[],[R[35]]]],[11,R[43],E,E,1,[[["k"],["self"]],["bool"]]],[11,"from",E,E,11,[[[T]],[T]]],[11,"into",E,E,11,[[],[U]]],[11,R[37],E,E,11,[[[U]],[R[35]]]],[11,R[44],E,E,11,[[["self"]],[T]]],[11,R[40],E,E,11,[[["self"]],[R[45]]]],[11,R[38],E,E,11,[[["self"]],[T]]],[11,R[39],E,E,11,[[],[R[35]]]],[11,"from",E,E,16,[[[T]],[T]]],[11,"into",E,E,16,[[],[U]]],[11,R[41],E,E,16,[[["self"]],[T]]],[11,R[42],E,E,16,[[[T],["self"]]]],[11,R[37],E,E,16,[[[U]],[R[35]]]],[11,R[44],E,E,16,[[["self"]],[T]]],[11,R[40],E,E,16,[[["self"]],[R[45]]]],[11,R[38],E,E,16,[[["self"]],[T]]],[11,R[39],E,E,16,[[],[R[35]]]],[11,R[43],E,E,16,[[["k"],["self"]],["bool"]]],[11,"from",E,E,12,[[[T]],[T]]],[11,"into",E,E,12,[[],[U]]],[11,R[37],E,E,12,[[[U]],[R[35]]]],[11,R[44],E,E,12,[[["self"]],[T]]],[11,R[40],E,E,12,[[["self"]],[R[45]]]],[11,R[38],E,E,12,[[["self"]],[T]]],[11,R[39],E,E,12,[[],[R[35]]]],[11,"from",E,E,13,[[[T]],[T]]],[11,"into",E,E,13,[[],[U]]],[11,R[41],E,E,13,[[["self"]],[T]]],[11,R[42],E,E,13,[[[T],["self"]]]],[11,R[37],E,E,13,[[[U]],[R[35]]]],[11,R[44],E,E,13,[[["self"]],[T]]],[11,R[40],E,E,13,[[["self"]],[R[45]]]],[11,R[38],E,E,13,[[["self"]],[T]]],[11,R[39],E,E,13,[[],[R[35]]]],[11,"from",E,E,14,[[[T]],[T]]],[11,"into",E,E,14,[[],[U]]],[11,R[37],E,E,14,[[[U]],[R[35]]]],[11,R[44],E,E,14,[[["self"]],[T]]],[11,R[40],E,E,14,[[["self"]],[R[45]]]],[11,R[38],E,E,14,[[["self"]],[T]]],[11,R[39],E,E,14,[[],[R[35]]]],[11,"from",E,E,15,[[[T]],[T]]],[11,"into",E,E,15,[[],[U]]],[11,R[37],E,E,15,[[[U]],[R[35]]]],[11,R[44],E,E,15,[[["self"]],[T]]],[11,R[40],E,E,15,[[["self"]],[R[45]]]],[11,R[38],E,E,15,[[["self"]],[T]]],[11,R[39],E,E,15,[[],[R[35]]]],[11,"from",E,E,17,[[[T]],[T]]],[11,"into",E,E,17,[[],[U]]],[11,R[37],E,E,17,[[[U]],[R[35]]]],[11,R[44],E,E,17,[[["self"]],[T]]],[11,R[40],E,E,17,[[["self"]],[R[45]]]],[11,R[38],E,E,17,[[["self"]],[T]]],[11,R[39],E,E,17,[[],[R[35]]]],[11,"from",R[46],E,18,[[[T]],[T]]],[11,"into",E,E,18,[[],[U]]],[11,R[37],E,E,18,[[[U]],[R[35]]]],[11,R[44],E,E,18,[[["self"]],[T]]],[11,R[40],E,E,18,[[["self"]],[R[45]]]],[11,R[38],E,E,18,[[["self"]],[T]]],[11,R[39],E,E,18,[[],[R[35]]]],[11,R[47],E,E,18,[[],["f"]]],[11,"from",E,E,2,[[[T]],[T]]],[11,"into",E,E,2,[[],[U]]],[11,R[41],E,E,2,[[["self"]],[T]]],[11,R[42],E,E,2,[[[T],["self"]]]],[11,R[37],E,E,2,[[[U]],[R[35]]]],[11,R[44],E,E,2,[[["self"]],[T]]],[11,R[40],E,E,2,[[["self"]],[R[45]]]],[11,R[38],E,E,2,[[["self"]],[T]]],[11,R[39],E,E,2,[[],[R[35]]]],[11,"from",E,E,19,[[[T]],[T]]],[11,"into",E,E,19,[[],[U]]],[11,R[37],E,E,19,[[[U]],[R[35]]]],[11,R[44],E,E,19,[[["self"]],[T]]],[11,R[40],E,E,19,[[["self"]],[R[45]]]],[11,R[38],E,E,19,[[["self"]],[T]]],[11,R[39],E,E,19,[[],[R[35]]]],[11,R[47],E,E,19,[[],["f"]]],[11,"from",E,E,4,[[[T]],[T]]],[11,"into",E,E,4,[[],[U]]],[11,R[37],E,E,4,[[[U]],[R[35]]]],[11,R[44],E,E,4,[[["self"]],[T]]],[11,R[40],E,E,4,[[["self"]],[R[45]]]],[11,R[38],E,E,4,[[["self"]],[T]]],[11,R[39],E,E,4,[[],[R[35]]]],[11,R[47],E,E,4,[[],["f"]]],[11,"from",E,E,5,[[[T]],[T]]],[11,"into",E,E,5,[[],[U]]],[11,R[37],E,E,5,[[[U]],[R[35]]]],[11,R[44],E,E,5,[[["self"]],[T]]],[11,R[40],E,E,5,[[["self"]],[R[45]]]],[11,R[38],E,E,5,[[["self"]],[T]]],[11,R[39],E,E,5,[[],[R[35]]]],[11,R[47],E,E,5,[[],["f"]]],[11,"from",E,E,7,[[[T]],[T]]],[11,"into",E,E,7,[[],[U]]],[11,R[37],E,E,7,[[[U]],[R[35]]]],[11,R[44],E,E,7,[[["self"]],[T]]],[11,R[40],E,E,7,[[["self"]],[R[45]]]],[11,R[38],E,E,7,[[["self"]],[T]]],[11,R[39],E,E,7,[[],[R[35]]]],[11,R[47],E,E,7,[[],["f"]]],[11,"from",E,E,6,[[[T]],[T]]],[11,"into",E,E,6,[[],[U]]],[11,R[37],E,E,6,[[[U]],[R[35]]]],[11,R[44],E,E,6,[[["self"]],[T]]],[11,R[40],E,E,6,[[["self"]],[R[45]]]],[11,R[38],E,E,6,[[["self"]],[T]]],[11,R[39],E,E,6,[[],[R[35]]]],[11,"from",E,E,20,[[[T]],[T]]],[11,"into",E,E,20,[[],[U]]],[11,R[37],E,E,20,[[[U]],[R[35]]]],[11,R[44],E,E,20,[[["self"]],[T]]],[11,R[40],E,E,20,[[["self"]],[R[45]]]],[11,R[38],E,E,20,[[["self"]],[T]]],[11,R[39],E,E,20,[[],[R[35]]]],[11,"from",E,E,3,[[[T]],[T]]],[11,"into",E,E,3,[[],[U]]],[11,R[41],E,E,3,[[["self"]],[T]]],[11,R[42],E,E,3,[[[T],["self"]]]],[11,R[37],E,E,3,[[[U]],[R[35]]]],[11,R[44],E,E,3,[[["self"]],[T]]],[11,R[40],E,E,3,[[["self"]],[R[45]]]],[11,R[38],E,E,3,[[["self"]],[T]]],[11,R[39],E,E,3,[[],[R[35]]]],[11,"from",R[48],E,21,[[[T]],[T]]],[11,"into",E,E,21,[[],[U]]],[11,R[37],E,E,21,[[[U]],[R[35]]]],[11,R[44],E,E,21,[[["self"]],[T]]],[11,R[40],E,E,21,[[["self"]],[R[45]]]],[11,R[38],E,E,21,[[["self"]],[T]]],[11,R[39],E,E,21,[[],[R[35]]]],[11,R[47],E,E,21,[[],["f"]]],[11,"from",E,E,8,[[[T]],[T]]],[11,"into",E,E,8,[[],[U]]],[11,R[37],E,E,8,[[[U]],[R[35]]]],[11,R[44],E,E,8,[[["self"]],[T]]],[11,R[40],E,E,8,[[["self"]],[R[45]]]],[11,R[38],E,E,8,[[["self"]],[T]]],[11,R[39],E,E,8,[[],[R[35]]]],[11,"from",E,E,9,[[[T]],[T]]],[11,"into",E,E,9,[[],[U]]],[11,R[41],E,E,9,[[["self"]],[T]]],[11,R[42],E,E,9,[[[T],["self"]]]],[11,R[37],E,E,9,[[[U]],[R[35]]]],[11,R[44],E,E,9,[[["self"]],[T]]],[11,R[40],E,E,9,[[["self"]],[R[45]]]],[11,R[38],E,E,9,[[["self"]],[T]]],[11,R[39],E,E,9,[[],[R[35]]]],[11,"from",E,E,10,[[[T]],[T]]],[11,"into",E,E,10,[[],[U]]],[11,R[37],E,E,10,[[[U]],[R[35]]]],[11,R[44],E,E,10,[[["self"]],[T]]],[11,R[40],E,E,10,[[["self"]],[R[45]]]],[11,R[38],E,E,10,[[["self"]],[T]]],[11,R[39],E,E,10,[[],[R[35]]]],[11,"eq","h2",E,1,[[[R[0]],["self"]],["bool"]]],[11,"ne",E,E,1,[[[R[0]],["self"]],["bool"]]],[11,"eq",E,E,16,[[["self"],[R[10]]],["bool"]]],[11,"ne",E,E,16,[[["self"],[R[10]]],["bool"]]],[11,"clone",E,E,1,[[["self"]],[R[0]]]],[11,"clone",R[46],E,3,[[["self"]],[R[49]]]],[11,"clone",E,E,2,[[["self"]],["self"]]],[11,"clone",R[48],E,9,[[["self"]],[R[49]]]],[11,"clone","h2",E,16,[[["self"]],[R[10]]]],[11,"clone",E,E,13,[[["self"]],["self"]]],[11,"from",E,E,0,[[[R[3]]],[R[3]]]],[11,"from",E,E,0,[[[R[0]]],[R[3]]]],[11,"from",E,E,1,[[["u32"]],[R[0]]]],[11,"drop",E,E,12,[[["self"]]]],[11,"default",R[46],E,3,[[],[R[49]]]],[11,"default",R[48],E,9,[[],[R[49]]]],[11,"fmt","h2",E,0,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,1,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",R[46],E,19,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,5,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,7,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,6,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,20,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,3,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,2,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,4,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,18,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",R[48],E,9,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,10,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,8,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,21,[[[R[50]],["self"]],[R[35]]]],[11,"fmt","h2",E,11,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,16,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,13,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,12,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,14,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,15,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,17,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,0,[[[R[50]],["self"]],[R[35]]]],[11,"fmt",E,E,1,[[[R[50]],["self"]],[R[35]]]],[11,"hash",E,E,16,[[["self"],["__h"]]]],[11,R[51],E,E,0,[[["self"]],["str"]]],[11,"poll",R[46],E,19,[[["self"]],["poll"]]],[11,"poll",E,E,4,[[["self"]],[[R[3]],["poll",[R[3]]]]]],[11,"poll",E,E,18,[[["self"]],["poll"]]],[11,"poll",E,E,5,[[["self"]],["poll"]]],[11,"poll",E,E,7,[[["self"]],["poll"]]],[11,"poll",R[48],E,21,[[["self"]],["poll"]]],[11,"poll",R[46],E,20,[[["self"]],[[R[1]],["poll",[R[1]]]]]],[11,"poll",R[48],E,8,[[["self"]],[["poll",[R[1],R[3]]],[R[3]],[R[1]]]]],[11,"poll","h2",E,12,[[["self"]],[[R[1]],["poll",[R[1]]]]]]],"p":[[3,"Error"],[3,"Reason"],[3,R[52]],[3,R[53]],[3,R[12]],[3,R[54]],[3,R[55]],[3,R[56]],[3,R[12]],[3,R[53]],[3,R[57]],[3,R[58]],[3,R[59]],[3,R[60]],[3,R[61]],[3,"Ping"],[3,R[62]],[3,"Pong"],[3,R[11]],[3,R[63]],[3,R[64]],[3,R[11]]]};
initSearch(searchIndex);addSearchOptions(searchIndex);