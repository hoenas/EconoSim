# econo-sim

Economy simulation written in Rust

# Basic Structure

```mermaid
graph TD;
    World -- owns --> Company
    World -- owns --> Marketplace
    Marketplace -- holds -->Offer
    Marketplace -- holds -->Order
    Producer -- creates -->Offer
    Consumer -- creates -->Order
    Company -- places -->Offer
    Company -- places -->Order
    Company -- owns -->Processor
    Company -- owns --> Resource
    Processor -- produces --> Resource
    Resource -- is consumed by --> Processor
    Offer -- includes --> Resource
```

Potential features:

- In- and outlux to the system
  - Basic resources must be placed into the market at (chaning?) rates
  - Sophisticated products must be consumed by end consumers
- Banking
  - Banks can lend money to customers
  - Banks get the money back with interest
  - Companys may fail with paying back
  - Banks may fail
- Shares
  - Companies can sell shares to get currency
  - Comapnies can be bought by other companies using shares
- AI agent
  - An AI agent controls a company
  - Agent success is measured with the following factors:
    - Company value:
      - All stock multiplied by current market value
      - Monopol bonus?
    - Current production rate
    - Currency influx
    - Currency
