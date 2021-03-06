0.5.0
-----
- Introduced new public `events` module
- Removed `Hyper` variant from `Error` type
- Bumped `http-endpoint` dependency to `0.2`
  - Introduced new `RequestError` type
- Bumped `websocket-util` dependency to `0.4`


0.4.0
-----
- Added support for properly handling "disconnect" messages
- Adjusted stream functionality to stream a single event at a time
- Removed `Option` wrapper from aggregate response type
- Removed `Aggregate::accumulated_volume` and `Aggregate::average_price`
  members
- Decreased tracing verbosity by one level
- Bumped `num-decimal` dependency to `0.2`


0.3.0
-----
- Introduced `Client::new` constructor
- Assume Eastern time when deserializing streamed market data
- Serialize stock aggregates in millisecond format
- Removed `Aggregate::open_price_today` member


0.2.0
-----
- Bumped `time-util` dependency to `0.2`
- Bumped `websocket-util` dependency to `0.3`
- Bumped `async-tungstenite` dependency to `0.4`


0.1.2
-----
- Made `Aggregate`, `Quote`, and `Trade` types publicly available
- Added `Event::symbol` and `Subscription::stock` methods


0.1.1
-----
- Added support for retrieving stock aggregates
- Added support for retrieving ticker news items
- Made stream event types serializable
- Use `time-util` crate for time related operations


0.1.0
-----
- Initial release
