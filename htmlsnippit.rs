let response: ~str = match reqtype[1]{
                        	"/"=>
                            fmt!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                             <doctype !html><html><head><title>Hello, Rust!</title>
                             <style>body { background-color: #111; color: #FFEEAA }
                                    h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                             </style></head>
                             <body>
                             <h1>Greetings, Rusty!</h1>
                             <p>Number of Visitors: %d </p>
                             </body></html>\r\n",  get_visitors()),

                        
                        	_=>"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n"+serve_page("." + reqtype[1])
                        
                        
                        
                        };
