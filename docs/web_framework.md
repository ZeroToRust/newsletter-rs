# Frame Work Used

Among the vast options of frame work rust provides it is important to choose a framework that allow for performance and efficiency even for large scale applications.

Rust provides a frame work like actix that is very performant and has a low latency with. Also actix consumes less resources like CPU and Memory as it uses asynchronous programming approach.

Even thought actix web has all these features we are going to take something simple which is axum that is beginner friendly .

## Comparing  Axum and Actix-web

| **Feature**                | **Axum**                                      | **Actix-web**                                |
|----------------------------|-----------------------------------------------|---------------------------------------------|
| **Performance**            | High performance, slightly slower than Actix | Extremely high performance, low latency     |
| **Ease of Use**            | Beginner-friendly, ergonomic APIs            | Steeper learning curve                      |
| **Middleware**             | Uses Tower and Tower-HTTP with rich features | Built-in middleware, fewer options          |
| **Error Handling**         | Strong compile-time guarantees               | Requires more manual effort                 |
| **Scalability**            | Scales well, uses more resources             | Highly scalable, low resource consumption   |
| **Threading Model**        | Hyper's threading model                      | Thread pool, avoids shared state            |
| **Community**              | Growing, modern design                       | Larger, more mature community               |
| **Use Case**               | Rich middleware and ergonomic APIs           | High performance and resource efficiency    |

> **Note**: 

 Even though Actix-web is still very efficient when it comes to scalability since it consumes less computer resources, Axum is also very efficient in catching errors(enforcing rust rules)  and easy to use and uses middle ware provided by tower and tower-http which has much more features than actix web for handling apis.

 <!-- We are going to be using axum in this project. -->

To better understand these differences and comparisons between axum and actix web visit the following sites [comparison with charts](https://medium.com/deno-the-complete-reference/rust-actix-vs-axum-hello-world-performance-e10a1c1419e0)
[others](https://aarambhdevhub.medium.com/actix-vs-axum-a-deep-dive-into-rusts-premier-web-frameworks-737f3de52fe5)

Further more visit their documentations [actix-web](https://docs.rs/actix-web/latest/actix_web/) and [axum](https://docs.rs/axum/latest/axum/)



