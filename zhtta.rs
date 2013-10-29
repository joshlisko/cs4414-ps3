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
// Version 0.3

extern mod extra;

use std::rt::io::*;
use std::rt::io::net::ip::SocketAddr;
use std::io::println;
use std::cell::Cell;
use std::{os, str, io};
use extra::arc;
use std::comm::*;
use extra::priority_queue;

static PORT:    int = 4414;
static IP: &'static str = "0.0.0.0";
static mut visitor_count: uint = 0;

struct sched_msg {
    stream: Option<std::rt::io::net::tcp::TcpStream>,
    filepath: ~std::path::PosixPath,
    filesize: uint
}


impl Ord<> for sched_msg {
    fn lt(&self, other: &sched_msg)-> bool {
        if(self.filesize>other.filesize){
            return true;
        }
        return false;
    }
    fn le(&self, other: &sched_msg)-> bool {
        if(self.filesize>=other.filesize){
            return true;
        }
        return false;
    }
    fn gt(&self, other: &sched_msg)-> bool {
        if(self.filesize<other.filesize){
            return true;
        }
        return false;
    }
    fn ge(&self, other: &sched_msg)-> bool {
        if(self.filesize<=other.filesize){
            return true;
        }
        return false;
    }

}


fn main() {
    let mut visitor_count: uint =0;
    let safe_visitor_count = arc::RWArc::new(visitor_count);


    let mut req_pq_other: priority_queue::PriorityQueue<sched_msg> = priority_queue::PriorityQueue::new();
    let mut shared_req_pq_other = arc::RWArc::new(req_pq_other);
    let add_pq_other = shared_req_pq_other.clone();
    let take_pq_other = shared_req_pq_other.clone();

    let mut req_pq_cville: priority_queue::PriorityQueue<sched_msg> = priority_queue::PriorityQueue::new();
    let mut shared_req_pq_cville = arc::RWArc::new(req_pq_cville);
    let add_pq_cville= shared_req_pq_cville.clone();
    let take_pq_cville = shared_req_pq_cville.clone();
    
    let (port, chan) = stream();
    let chan = SharedChan::new(chan);
    



    let ip = match FromStr::from_str(IP) { Some(ip) => ip, 
                                           None => { println(fmt!("Error: Invalid IP address <%s>", IP));
                                                     return;},
                                         };
                                         
    let socket = net::tcp::TcpListener::bind(SocketAddr {ip: ip, port: PORT as u16});

    //listen to ip
    println(fmt!("Listening on %s:%d ...", ip.to_str(), PORT));

    //parse peer IP
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
    let secondIndex = tempIP.find_str(".");
    let secondElement = tempIP.slice(0, secondIndex.unwrap());
    println(fmt!("Ip Index 2 : %?", secondElement));

    //Determine location of user
    let inCville = ((firstElement == "128" && firstElement == "143") || (firstElement == "137" && secondElement == "54") || /*(firstElement == "127") ||*/ (firstElement == "192"));



    // dequeue file requests, and send responses.
    // FIFO
    let inCville_copy = inCville;
    do spawn {
        let (sm_port, sm_chan) = stream();
        // a task for sending responses.
        
        loop {
            port.recv(); // wait for arrving notification
            if(inCville){
                do take_pq_cville.write |pq| {
                    if ((*pq).len() > 0) {
                        // LIFO didn't make sense in service scheduling, so we modify it as FIFO by using shift_opt() rather than pop().
                        let tf_opt: Option<sched_msg> = (*pq).maybe_pop();
                        let tf = tf_opt.unwrap();
                        println(fmt!("Serving Cville file: %?", tf.filepath.filename().unwrap()));
                        println(fmt!("Cville queue size: %u", (*pq).len()));
                        //println(fmt!("shift from queue, size: %ud", (*vec).len()));
                        sm_chan.send(tf); // send the request to send-response-task to serve.
                    }
                }
                let mut tf: sched_msg = sm_port.recv(); // wait for the dequeued request to handle
                match io::read_whole_file(tf.filepath) { // killed if file size is larger than memory size.
                    Ok(file_data) => {
                        //println(fmt!("begin serving file: %?", tf.filepath.filename().unwrap()));
                        // A web server should always reply a HTTP header for any legal HTTP request.
                        tf.stream.write("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream; charset=UTF-8\r\n\r\n".as_bytes());
                        tf.stream.write(file_data);
                        //println(fmt!("finish file [%?]", tf.filepath));
                    }
                    Err(err) => {
                        println(err);
                    }
                }
            } //end if

            else{
                do take_pq_other.write |pq| {
                    if ((*pq).len() > 0) {
                        // LIFO didn't make sense in service scheduling, so we modify it as FIFO by using shift_opt() rather than pop().
                        let tf_opt: Option<sched_msg> = (*pq).maybe_pop();
                        let tf = tf_opt.unwrap();
                        println(fmt!("Serving Other file: %?", tf.filepath.filename().unwrap()));
                        println(fmt!("Other queue size: %u", (*pq).len()));
                        //println(fmt!("shift from queue, size: %ud", (*vec).len()));
                        sm_chan.send(tf); // send the request to send-response-task to serve.
                    }
                }
                let mut tf: sched_msg = sm_port.recv(); // wait for the dequeued request to handle
                match io::read_whole_file(tf.filepath) { // killed if file size is larger than memory size.
                    Ok(file_data) => {
                        //println(fmt!("begin serving file: %?", tf.filepath.filename().unwrap()));
                        // A web server should always reply a HTTP header for any legal HTTP request.
                        tf.stream.write("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream; charset=UTF-8\r\n\r\n".as_bytes());
                        tf.stream.write(file_data);
                        //println(fmt!("finish file [%?]", tf.filepath));
                    }
                    Err(err) => {
                        println(err);
                    }
                }
            }



        }
    }

    



    //start listening
    let mut acceptor = acceptor1.unwrap();
    for stream in acceptor.incoming() {
        let safe_visitor_count_local = safe_visitor_count.clone();
        let stream = Cell::new(stream);
        //println(fmt!("new stream: %?", stream));
        // Start a new task to handle the each connection
        let child_chan = chan.clone();
        let child_add_pq_cville = add_pq_cville.clone();
        let child_add_pq_other = add_pq_other.clone();

        do spawn {
            
            do safe_visitor_count_local.write |visitor_count|{
                *visitor_count +=1;
            }

            
            let mut stream = stream.take();
            let mut buf = [0, ..500];
            stream.read(buf);
            let request_str = str::from_utf8(buf);
            
            let req_group : ~[&str]= request_str.splitn_iter(' ', 3).collect();
            if req_group.len() > 2 {
                let path = req_group[1];
                //println(fmt!("Request for path: \n%?", path));
                
                let file_path = ~os::getcwd().push(path.replace("/../", ""));
                if !os::path_exists(file_path) || os::path_is_dir(file_path) {
                    //println(fmt!("Request received:\n%s", request_str));

                    let mut visitor_count_copy: uint = 0;
                    do safe_visitor_count_local.read |visitor_count| {
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
                    // Requests scheduling
                    let file_size = std::path::Path(file_path.to_str()).stat().unwrap().st_size as uint;
                    let msg: sched_msg = sched_msg{stream: stream, filepath: file_path.clone(), filesize: file_size};
                    let (sm_port, sm_chan) = std::comm::stream();
                    sm_chan.send(msg);
                    
                    if(inCville){
                        do child_add_pq_cville.write |pq| {
                            let msg = sm_port.recv();
                            println(fmt!("add to Cville queue: %?", msg.filepath.filename().unwrap()));
                            (*pq).push(msg); // enqueue new request.
                            println(fmt!("Cville queue size: %u", (*pq).len()));
                        }
                        child_chan.send(""); //notify the new arriving request.
                    }

                    else{
                        do child_add_pq_other.write |pq| {
                            let msg = sm_port.recv();
                            println(fmt!("add to Other queue: %?", msg.filepath.filename().unwrap()));
                            (*pq).push(msg); // enqueue new request.
                            println(fmt!("Other queue size: %u", (*pq).len()));
                        }
                        child_chan.send("");
                    }

                    //println(fmt!("get file request: %?", file_path));
                }
            }
            //println!("connection terminates")
        }
    }
}
