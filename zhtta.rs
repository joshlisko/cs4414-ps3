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
use extra::priority_queue;


static PORT:    int = 4414;
static IP: &'static str = "0.0.0.0";



struct sched_msg {
    stream: Option<std::rt::io::net::tcp::TcpStream>,
    filepath: ~std::path::PosixPath,
    filesize: uint
}

impl Ord<> for sched_msg {
	fn lt(&self, other: &sched_msg)-> bool {
		if(self.filesize<other.filesize){
			return true;
		}
		return false;
	}
	fn le(&self, other: &sched_msg)-> bool {
		if(self.filesize<=other.filesize){
			return true;
		}
		return false;
	}
	fn gt(&self, other: &sched_msg)-> bool {
		if(self.filesize>other.filesize){
			return true;
		}
		return false;
	}
	fn ge(&self, other: &sched_msg)-> bool {
		if(self.filesize>=other.filesize){
			return true;
		}
		return false;
	}

}

fn main() {

    let mut visitor_count: uint = 0;
    let safe_visitor_count = arc::RWArc::new(visitor_count);
    

    let req_pq_other: priority_queue::PriorityQueue<sched_msg> = priority_queue::PriorityQueue::new();
    let shared_req_pq_other = arc::RWArc::new(req_pq_other);
    let add_pq_other = shared_req_pq_other.clone();
    let take_pq_other = shared_req_pq_other.clone();

    let req_pq_cville: priority_queue::PriorityQueue<sched_msg> = priority_queue::PriorityQueue::new();
    let shared_req_pq_cville = arc::RWArc::new(req_pq_cville);
    let add_pq_cville= shared_req_pq_cville.clone();
    let take_pq_cville = shared_req_pq_cville.clone();
    
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

     let inCville = ((firstElement == "128" && firstElement == "143") || (firstElement == "137" && secondElement == "54") || (firstElement == "127"));
    // add file requests into queue.

    do spawn {
        loop{

            if(inCville){
                do add_pq_cville.write |pq| {
                    if(true){
                        println("getting to port.peek");
                        let tf:sched_msg = port.recv();
                        println("getting after port.recv");
                        (*pq).push(tf);
                    }
                    println(fmt!("add to queue, size: %ud", (*pq).len()));
                }
            }

            else{
                do add_pq_other.write |pq| {
                    //port.recv() will block the code and keep locking the RWArc, so we simply use peek() to check if there's message to recv.
                    //But a asynchronous solution will be much better.
                    //if (port.peek()) { //this wasnt working....
                    if(true){
                        println("getting to port.peek");
                        let tf:sched_msg = port.recv();
                        println("getting after port.recv");
                        (*pq).push(tf);
                    }
                        println(fmt!("add to queue, size: %ud", (*pq).len()));
                    
                }
            }
        }
    }
    



    // take file requests from queue, and send a response.
    //FIFO
    do spawn {
        loop{
            if(inCville){
                do take_pq_cville.write |pq| {
                    if ((*pq).len() > 0) {
                        let mut tf = (*pq).pop();
                        println(fmt!("shift from queue, size: %ud", (*pq).len()));
			//Code to get the size of a file
			//let filerequestpath = tf.filepath.to_str();
			//let file_size = std::path::Path(filerequestpath).stat().unwrap().st_size as uint;
			//println(fmt!("Size: %?", file_size));
                        match io::read_whole_file(tf.filepath) { // killed if file size is larger than memory size.
                            Ok(file_data) => {
                                println(fmt!("begin serving file to Cville request [%?]", tf.filepath));
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

            else{
                do take_pq_other.write |pq| {
                    if ((*pq).len() > 0) {
                         let mut tf = (*pq).pop();
                        println(fmt!("shift from queue, size: %ud", (*pq).len()));
			println(fmt!("Size: %?", io::read_whole_file(tf.filepath)));
			//println(fmt!("File size test: %?", std::path::PosixPath::get_size(tf.filepath))); 
                        match io::read_whole_file(tf.filepath) { // killed if file size is larger than memory size.
                            Ok(file_data) => {
                                println(fmt!("begin serving file to Other request [%?]", tf.filepath));
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
                    let filerequestpath = file_path.to_str();
					let file_size = std::path::Path(filerequestpath).stat().unwrap().st_size as uint;
                    let msg: sched_msg = sched_msg{stream: stream, filepath: file_path.clone(), filesize: file_size};
                    child_chan.send(msg);
                    
                    println(fmt!("get file request: %?", file_path));
                }
            }
            println!("connection terminates")
        }
    }
}

