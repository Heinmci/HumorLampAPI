# State of Mind Lamp

The goal of this project it to be able to see the world's state of mind, or more precisely, the french and the english speaking people of the world.

To meet that goal, we've set up two streams to the Twitter API, one with english terms and the other with french terms.
We count the number of times that a word associated to being scared, being sad and being happy are tweeted and we determine the one that is the most present, kepping a history of 8 minutes at a time.
We also use the Twitter API to retrieve the current biggest trend.

On the micro-controller side, we have a LED with 8 lights, one button and one screen.
The LED displays the worlds state of mind in the last 8 minutes and update the current minute every 5 seconds.

The button is used to switch between the french mood and the english one.

Lastly, the display is used to display what we are currently displaying (French / English) as well as their biggest current trend.

The colour code is the following: 

Blue -> Happy

Red -> Scared

Green -> Sad

Pink / Purple -> Scared + Happy

White -> Nothing of note

To run the API, modify src/keys.rs with your Twitter API keys and type the following command: 
```
cargo run --release
```


