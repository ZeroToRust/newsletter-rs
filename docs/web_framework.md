# Framework Used

Among the vast options of framework rust provides it is important to choose a framework that allow for performance and efficiency even for large scale applications.

Rust provides a framework like actix that is very performant and has a low latency with. Also actix consumes less resources like CPU and Memory as it uses asynchronous programming approach.
## Features that actix-web offers
-  It uses a threadpool that is implemented by default to handle multiple requests asynchronously.
-  It ensures performance by avoiding shared state between threads.
-  It provides a middleware that allows for efficient API interactions.
> **Note**: 

Even though axum is also very efficient in catching errors (enforcing Rust rules), easy to use, and uses middleware provided by tower and tower-http, which has many more features than actix-web for handling APIs, actix-web is still very efficient when it comes to scalability since it consumes fewer computer resources.
To better understand these differences and comparisons between axum and actix web visit the following sites [comparison with charts](https://medium.com/deno-the-complete-reference/rust-actix-vs-axum-hello-world-performance-e10a1c1419e0)
[others](https://aarambhdevhub.medium.com/actix-vs-axum-a-deep-dive-into-rusts-premier-web-frameworks-737f3de52fe5)

Further more visit their documentations [actix-web](https://docs.rs/actix-web/latest/actix_web/) and [axum](https://docs.rs/axum/latest/axum/)

