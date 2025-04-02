# Frame Work Used

Among the vast options of frame work rust provides it is important to choose a framework that allow for performance and efficiency even for large scale applications.

Rust provides a frame work like actix that is very performant and has a low latency with. Also actix consumes less resources like CPU and Memory as it uses asynchronous programming approach.

## Features that actix-web offer

-  It uses a threadpool that is implemented by default to handle multiple request asynchronously .
-  It ensures performance by avoiding shared state between threads.
-  It provides a middleware that allow for efficient api interactions.

## Features of axum
It has or allows us to use the following features for free since it built on top of tower and tower-http
- Timeouts
- Tracing 
- Compression
- And Authorization
> **Note**: 

 Even though Actix-web is still very efficient when it comes to scalability since it consumes less computer resources, Axum is also very efficient in catching errors(enforcing rust rules)  and easy to use and uses middle ware provided by tower and tower-http which has much more features than actix web for handling apis.

 We are going to be using axum in this project.

To better understand these differences and comparisons between axum and actix web visit the following sites [comparison with charts](https://medium.com/deno-the-complete-reference/rust-actix-vs-axum-hello-world-performance-e10a1c1419e0)
[others](https://aarambhdevhub.medium.com/actix-vs-axum-a-deep-dive-into-rusts-premier-web-frameworks-737f3de52fe5)

Further more visit their documentations [actix-web](https://docs.rs/actix-web/latest/actix_web/) and [axum](https://docs.rs/axum/latest/axum/)

