# mosam 🌤️

A simple Rust command-line tool to fetch today's weather for any city using the OpenWeather API.

## Usage

```bash
mosam <city_name>
```

Example:

```bash
mosam Islamabad
```

Output:

```
Islamabad
Temperature: 30°C
Humidity: 60%
Condition: clear sky
```

## Setup

1. Clone the repo:

   ```bash
   git clone https://github.com/khurrambhutto/mosam.git
   cd mosam
   ```

2. Add your OpenWeather API key in a `.env` file:

   ```
   WEATHER_API_KEY=your_api_key_here
   ```

3. Build and run:

   ```bash
   cargo build --release
   cargo run -- <city>
   ```

## License

MIT
