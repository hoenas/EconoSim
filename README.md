# EconoSim
Economy simulation written in Rust

# Basic Structure

~~~mermaid
graph TD;
    World -- owns --> Company
    World -- owns --> Marketplace
    Marketplace-- holds -->Offer
    Company -- places offer -->Marketplace
    Company -- accepts offer -->Marketplace
    Company -- owns -->Processor
    Company -- owns --> Resource
    Processor -- produces --> Resource
    Resource -- is consumed by --> Processor
    Offer -- includes --> Resource
~~~
