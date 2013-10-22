//
// zhtta.rs
//
// Running on Rust 0.8
//
// Starting code for PS3
// 
// Note: it would be very unwise to run this server on a machine that is
// on the Internet and contains any sensitive files!
//
// University of Virginia - cs4414 Fall 2013
// Weilin Xu and David Evans
// Version 0.2

extern mod extra;

use std::rt::io::*;
use std::rt::io::net::ip::SocketAddr;
use std::io::println;
use std::cell::Cell;
use std::{os, str, io};
use extra::arc;
use std::comm::*;

use std::rt::io::*;
use std::rt::io::net::ip::SocketAddr;
use std::io::println;
use std::from_str::FromStr;


static PORT:    int = 4414;
static IP: &'static str = "0.0.0.0";


struct sched_msg {
    stream: Option<std::rt::io::net::tcp::TcpStream>,
    filepath: ~std::path::PosixPath
}

fn main() {
    let mut visitor_count: uint = 0;
    let safe_visitor_count = arc::RWArc::new(visitor_count);
    

    let req_vec: ~[sched_msg] = ~[];
    let shared_req_vec = arc::RWArc::new(req_vec);
    let add_vec = shared_req_vec.clone();
    let take_vec = shared_req_vec.clone();
    
    let (port, chan) = stream();
    let chan = SharedChan::new(chan);
    

    let ip = match FromStr::from_str(IP) { Some(ip) => ip, 
                                           None => { println(fmt!("Error: Invalid IP address <%s>", IP));
                                                     return;},
                                         };
   
    let socket = net::tcp::TcpListener::bind(SocketAddr {ip: ip, port: PORT as u16});

    let mut user_IP;
    let mut acceptor1 = socket.listen();
    match acceptor1.accept() {
            Some(s) => { let mut stream = s;
                         match stream.peer_name() {
                            Some(pn) => {user_IP = pn.to_str(); println(fmt!("Peer address: %s", user_IP));},
                            None => {user_IP = ~"";}
                         }
                       },
            None => {user_IP = ~"";}
        }

    let firstIndex = user_IP.find_str(".");
    let firstElement = user_IP.slice(0, firstIndex.unwrap());
    println(fmt!("IP Index 1: %?", firstElement));
    let tempIP = user_IP.slice(firstIndex.unwrap()+1, user_IP.len()-1);
    //println(fmt!("%?", tempIP));
    let secondIndex = tempIP.find_str(".");
    let secondElement = tempIP.slice(0, secondIndex.unwrap());
    println(fmt!("Ip Index 2 : %?", secondElement));
    // add file requests into queue.
    do spawn {
        loop{
            do add_vec.write |vec| {
                let tf:sched_msg = port.recv();
                (*vec).push(tf);
                println("add to queue");
                
                //Supposedly better code but is buggy so I commented it out

                //port.recv() will block the code and keep locking the RWArc, so we simply use peek() to check if there's message to recv.
                //But a asynchronous solution will be much better.
                //if (port.peek()) {
                    //let tf:sched_msg = port.recv();
                    //(*vec).push(tf);
                    //println(fmt!("add to queue, size: %ud", (*vec).len()));
                //}

                //End supposedly better code 
            }
        }
    }
    
    // take file requests from queue, and send a response.
    //FIFO
    do spawn {
        loop{
            do take_vec.write |vec| {
                if ((*vec).len() > 0) {
                    // FILO didn't make sense in service scheduling, so we modify it as FIFO by using shift_opt() rather than pop().
                     let tf_opt: Option<sched_msg> = (*vec).shift_opt();
                     let mut tf = tf_opt.unwrap();
                    println(fmt!("shift from queue, size: %ud", (*vec).len()));

                    match io::read_whole_file(tf.filepath) { // killed if file size is larger than memory size.
                        Ok(file_data) => {
                            println(fmt!("begin serving file [%?]", tf.filepath));
                            // A web server should always reply a HTTP header for any legal HTTP request.
                            tf.stream.write("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream; charset=UTF-8\r\n\r\n".as_bytes());
                            tf.stream.write(file_data);
                           println(fmt!("finish file [%?]", tf.filepath));
                        }
                        Err(err) => {
                            println(err);
                        }
                    } 
                }
            }
        }
    }
    let ip = match FromStr::from_str(IP) { Some(ip) => ip, 
                                           None => { println(fmt!("Error: Invalid IP address <%s>", IP));
                                                     return;},
                                         };
                                         
    let socket = net::tcp::TcpListener::bind(SocketAddr {ip: ip, port: PORT as u16});
    
    println(fmt!("Listening on tcp port %d ...", PORT));
   // let mut acceptor = socket.listen().unwrap();

    let mut acceptor = acceptor1.unwrap();
    // we can limit the incoming connection count.
    //for stream in acceptor.incoming().take(10 as uint) {
    for stream in acceptor.incoming() {
        let safe_visitor_count_local = safe_visitor_count.clone(); //create a local copy

        let stream = Cell::new(stream);
        
        // Start a new task to handle the connection
        let child_chan = chan.clone();
        do spawn {
            
        //update visitor count safely
        do safe_visitor_count_local.write |visitor_count| {
            *visitor_count +=1;
        }
         

            
            let mut stream = stream.take();
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            
            let req_group : ~[&str]= request_str.splitn_iter(' ', 3).collect();
            if req_group.len() > 2 {
                let path = req_group[1];
                println(fmt!("Request for path: \n%?", path));
                
                let file_path = ~os::getcwd().push(path.replace("/../", ""));
                if !os::path_exists(file_path) || os::path_is_dir(file_path) {
                    println(fmt!("Request received:\n%s", request_str));

                    //store the current count to a copy value
                    let mut visitor_count_copy: uint = 0;
                    do safe_visitor_count_local.read |visitor_count|{
                        visitor_count_copy = *visitor_count;
                    }

                    let response: ~str = fmt!(
                        "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                         <doctype !html><html><head><title>Hello, Rust!</title>
                         <style>body { background-color: #111; color: #FFEEAA }
                                h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                                h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                         </style></head>
                         <body>
                         <h1>Greetings, Krusty!</h1>
                         <h2>Visitor count: %u</h2>
                         </body></html>\r\n", visitor_count_copy);

                    stream.write(response.as_bytes());
                }
                else {
                    // may do scheduling here
                    let msg: sched_msg = sched_msg{stream: stream, filepath: file_path.clone()};
                    child_chan.send(msg);
                    
                    println(fmt!("get file request: %?", file_path));
                }
            }
            println!("connection terminates")
        }
    }
}
