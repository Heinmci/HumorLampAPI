#include <Wire.h>
#include "rgb_lcd.h"
#include <ESP8266WiFi.h>
#include <ESP8266HTTPClient.h>
#include <Button.h>
#include "FastLED.h"

#define RED_COLOR 0
#define GREEN_COLOR 50
#define BLUE_COLOR 0
#define FRANCE_INDEX 0
#define EN_US_INDEX 1
#define COUNTRIES_COUNT 2
#define NUM_LEDS 8
#define LED_PIN 0
#define BUTTON_PIN 2


struct country {
  char *api_method;
  char *api_mood;
  char *c_name;
};
rgb_lcd lcd;
int *cur_country_index;     //Current selected country index
struct country **countries; //Countries struct array
const char *api_port = "8080";
const char *api_server = "10.33.3.182";
const unsigned long callInterval = 1000 * 30;
const unsigned long moodInterval = 1000 * 5;
unsigned long lastCall;
unsigned long lastMood;
CRGB leds[NUM_LEDS];

void setup()
{
  Serial.begin(9600); // define I/O speed
  cur_country_index = (int*)calloc (1, sizeof(int));
  countries = (struct country **)calloc(COUNTRIES_COUNT, sizeof(country));
  lastCall = 0;

  init_country_structs(countries, COUNTRIES_COUNT);
  init_display(&lcd);
  init_LED();
  delay(500);
  connect_wifi("ESGI", "Reseau-GES", &lcd);
  delay(500);
  pinMode(BUTTON_PIN, INPUT);
  delay(500);
}
void loop()
{
  delay(100);
  int button_state;
  unsigned long currentTime = millis();

  //CALLING Twitter API
  if (WiFi.status() == WL_CONNECTED && (currentTime - lastCall > callInterval))
  {
    lastCall = currentTime;

    char query_buff[128];
    const struct country *cur_country = countries[*cur_country_index];
    const char *api_method = cur_country->api_method;
    const char *country_name = cur_country->c_name;

    snprintf (query_buff, sizeof(query_buff), "http://%s:%s/%s", api_server, api_port, api_method);
    HTTPClient http;
    http.begin(query_buff);
    int httpCode = http.GET();

    if (httpCode > 0)
    {
      const String tag = http.getString();
      show_tag_in_country(country_name, tag, &lcd);
    }
    else
    {
      write_chars_to_lcd("API call failed!", &lcd);
    }

    http.end();
  }
  //Calling Mood API
  if (WiFi.status() == WL_CONNECTED && (currentTime - lastMood > moodInterval))
  {
    lastMood = currentTime;
    char query_buff[128];
    const struct country *cur_country = countries[*cur_country_index];
    const char *api_mood = cur_country->api_mood;
    
    snprintf (query_buff, sizeof(query_buff), "http://%s:%s/%s", api_server, api_port, api_mood);
    HTTPClient http;  //Declare an object of class HTTPClient

    http.begin(query_buff);
    int httpCode = http.GET();
    if (httpCode > 0) {

      String input = http.getString();

      Serial.println(input);

      char* rgb = (char*) malloc(96 * sizeof(char));
      input.toCharArray(rgb, 96);

      char* token;
      int i = 0;
      token = strtok(rgb, ",");
      while (i < 8) {
        int r = atoi(token);
        token = strtok(NULL, ",");

        int g = atoi(token);
        token = strtok(NULL, ",");

        int b = atoi(token);
        token = strtok(NULL, ",");
        leds[i].setRGB(r, g, b);
        i++;
        FastLED.show();
      }
    }
    http.end();

  }

  //CHECKING BUTTON STATE
  button_state = digitalRead(BUTTON_PIN);

  if (button_state == LOW) //Button pressed
  {
    Serial.println("button");
    switch_country_index(cur_country_index, COUNTRIES_COUNT);
  }
}
void write_chars_to_lcd(char *chars, rgb_lcd *lcd)
{
  lcd->clear();
  lcd->setCursor(0, 0);
  lcd->print(chars);
}
void init_display(rgb_lcd *lcd)
{
  lcd->begin(16, 2);
  lcd->setRGB(RED_COLOR, GREEN_COLOR, BLUE_COLOR);
  lcd->display();
  lcd->clear();
}
void connect_wifi(char *ssid, char *password, rgb_lcd *lcd)
{
  WiFi.begin(ssid, password);
  while (WiFi.status() != WL_CONNECTED)
  {
    lcd->print("Connecting");
    for (int i = 0; i < 3; i++) {
      delay(500);
      lcd->print(".");
    }
    delay(1000);
    lcd->clear();
  }
  lcd->clear();
}
void show_tag_in_country(const char *country, const String tag, rgb_lcd *lcd)
{
  lcd->clear();
  lcd->setCursor(0, 0);
  lcd->print(country);
  lcd->setCursor(0, 1);
  lcd->print(tag);
}
void switch_country_index(int *selected_index, int country_count)
{
  if (selected_index == NULL)
  {
    selected_index = (int*)calloc(1, sizeof(int));
  }

  if (*selected_index >= country_count - 1)
  {
    *selected_index = 0;
  }
  else
  {
    ++ (*selected_index);
  }
}
void init_country_structs(struct country **country_array, int country_count)
{
  if (country_array == NULL)
  {
    country_array = (struct country **)calloc(country_count, sizeof(struct country));
  }
  for (int i = 0; i < country_count; i++)
  {
    struct country copy_struct;
    memset(&copy_struct, 0, sizeof(struct country));
    country_array[i] = (struct country *)calloc (1, sizeof(struct country));
    memcpy(country_array[i], &copy_struct, sizeof(struct country*)); //
  }
  country_array[FRANCE_INDEX]->c_name = "France";
  country_array[FRANCE_INDEX]->api_method = "france_trend";
  country_array[FRANCE_INDEX]->api_mood = "france_mood";

  /*country_array[PARIS_INDEX]->c_name = "Paris";
    country_array[PARIS_INDEX]->api_method = "paris_trend";*/
  country_array[EN_US_INDEX]->c_name = "EN-US";
  country_array[EN_US_INDEX]->api_method = "english_trend";
  country_array[EN_US_INDEX]->api_mood = "english_mood";
}

void init_LED() {
  FastLED.addLeds<WS2812B, LED_PIN, RGB>(leds, NUM_LEDS);
  FastLED.setBrightness(100);
}

