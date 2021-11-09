/** 
 * FastLED 
 */
#include "FastLED.h"

#define LED_TYPE WS2812B
#define COLOR_ORDER GRB
#define DATA_PIN_1 2
#define NUM_LEDS_1 138
#define DATA_PIN_2 3
#define NUM_LEDS_2 138
#define BRIGHTNESS_MAX 256
#define BRIGHTNESS_STEPS BRIGHTNESS_MAX / 32
#define DELAY_MAX 100
#define DELAY_STEPS DELAY_MAX / 50

#define RED_MAX 60
#define RED_NUM_MIN 30
#define RED_NUM_FADE 30


/** 
 * Rotary Encoder Inputs 
 */
#define BRIGHTNESS_PIN_CLOCK 10   //BRIGHTNESS_PIN_CLOCK
#define BRIGHTNESS_PIN_DATA 11    //BRIGHTNESS_PIN_DATA
#define BRIGHTNESS_PIN_PUSH 12    //BRIGHTNESS_PIN_PUSH 
#define DELAY_PIN_CLOCK 5   //BRIGHTNESS_PIN_CLOCK
#define DELAY_PIN_DATA 6    //BRIGHTNESS_PIN_DATA
#define DELAY_PIN_PUSH 7    //BRIGHTNESS_PIN_PUSH 


/**
 * Global Variables
 */
int brightness = BRIGHTNESS_MAX / 2;
int delay_value = DELAY_MAX / 10;
// int positionval;  
bool switchval_brightness; 
bool switchval_delay; 
int mrotateLast_brightness;  
int mrotateLast_delay;  
int mrotate_brightness;
int mrotate_delay;

// This is an array of leds.
CRGB leds1[NUM_LEDS_1];
CRGB leds2[NUM_LEDS_2];


/**
 * setup functions
 */
void setup_rotary_encoder() {
  Serial.println("** Init rotary encoder");
  /*  Set encoder pins as inputs with pullups. If you use the Encoder Module, you don't need 
   *  pullups for BRIGHTNESS_PIN_CLOCK and BRIGHTNESS_PIN_DATA, only for the BRIGHTNESS_PIN_PUSH button.*/
  pinMode(BRIGHTNESS_PIN_CLOCK,INPUT_PULLUP);
  pinMode(BRIGHTNESS_PIN_DATA,INPUT_PULLUP);
  pinMode(BRIGHTNESS_PIN_PUSH, INPUT_PULLUP);

  pinMode(DELAY_PIN_CLOCK,INPUT_PULLUP);
  pinMode(DELAY_PIN_DATA,INPUT_PULLUP);
  pinMode(DELAY_PIN_PUSH, INPUT_PULLUP);

  //Here we activate pin change interruptions on pin D8 and D9 with PCINT0 and PCINT1
  PCICR |= (1 << PCIE0);      //enable PCMSK0 scan                                                 
  PCMSK0 |= (1 << PCINT2);    // Pin 10
  PCMSK0 |= (1 << PCINT3);    // Pin 11
  PCMSK0 |= (1 << PCINT4);    // Pin 12

  PCICR |= (1 << PCIE2);      //enable PCMSK2 scan 
  PCMSK2 |= (1 << PCINT21);   // Pin 5
  PCMSK2 |= (1 << PCINT22);   // Pin 6
  PCMSK2 |= (1 << PCINT23);   // Pin 7
   
  // Read the initial state of BRIGHTNESS_PIN_CLOCK pin (it could be HIGH or LOW)
  mrotateLast_brightness = digitalRead(BRIGHTNESS_PIN_CLOCK); 
  mrotateLast_delay = digitalRead(DELAY_PIN_CLOCK); 
}

void setup_fastled() {
  Serial.println("** Init FastLed");
  delay(1000);  // sanity check delay - allows reprogramming if accidently blowing power w/leds
  pinMode(LED_BUILTIN, OUTPUT);

  FastLED.addLeds<LED_TYPE, DATA_PIN_1, COLOR_ORDER>(leds1, NUM_LEDS_1);
  FastLED.addLeds<LED_TYPE, DATA_PIN_2, COLOR_ORDER>(leds2, NUM_LEDS_2);
  FastLED.setBrightness(brightness);

  lightsOff();
  delay(500);
  lightsOn();
}

void setup() {
  delay(500);
  Serial.begin(9600);   // Setup Serial Monitor
  Serial.println("");
  Serial.println("*** SETUP ***");

  setup_rotary_encoder();
  setup_fastled();

  print_brightness();
  print_delay();
  Serial.println("*** SETUP DONE ***");
  Serial.println("");
}


/**
 * interrupt handler for the rotary encoder
 */
ISR (PCINT0_vect) {
  readencoder_brightness(); 
  
  if(readswitch_brightness() == 1){
    doBrightnessClick();
  }
}

ISR (PCINT2_vect) {
  readencoder_delay();
  
  if(readswitch_delay() == 1){
    doDelayClick();
  }
}


/**
 * business logic
 */
void doBrightnessClick() {
  Serial.println(">> brightness button push");
}

void doBrightnessLeft() {
  if (brightness > 0) {
    brightness -= BRIGHTNESS_STEPS;
    FastLED.setBrightness(brightness);
  }
  Serial.println (">> brightness rotated counter-clockwise");
}

void doBrightnessRight() {
  if (brightness < BRIGHTNESS_MAX) {
    brightness += BRIGHTNESS_STEPS;
    if (brightness == BRIGHTNESS_MAX) {
      FastLED.setBrightness(brightness - 1);
    } else {
      FastLED.setBrightness(brightness);
    }
    
  }
  Serial.println (">> brightness rotated clockwise");
}

void doDelayClick() {
  Serial.println(">> delay button push");
}

void doDelayLeft() {
  if (delay_value > 0) {
    delay_value -= DELAY_STEPS;
  }
  Serial.println (">> delay rotated counter-clockwise");
}

void doDelayRight() {
  if (delay_value < DELAY_MAX) {
    delay_value += DELAY_STEPS;
  }
  Serial.println (">> delay rotated clockwise");
}

void print_brightness() {
  Serial.print("Brightness: ");
  Serial.println(brightness);
}

void print_delay() {
  Serial.print("delay: ");
  Serial.println(delay_value);
}

/**
 * rotary encoder functions
 */
int readencoder_brightness(){
  cli();  // disable / pause interrupts
  mrotate_brightness = digitalRead(BRIGHTNESS_PIN_CLOCK);
  if (mrotate_brightness != mrotateLast_brightness){ //knob is rotating
    if (digitalRead(BRIGHTNESS_PIN_DATA) != mrotate_brightness) {  //switch A changed first -> rotating BRIGHTNESS_PIN_CLOCKwise
      doBrightnessLeft();
    } else {// switch B changed first -> rotating counterBRIGHTNESS_PIN_CLOCKwise
      doBrightnessRight();
    }
     
    print_brightness();
  }
  mrotateLast_brightness = mrotate_brightness; 
  
  return brightness;
}

int readencoder_delay(){
  cli();  // disable / pause interrupts
  mrotate_delay = digitalRead(DELAY_PIN_CLOCK);
  if (mrotate_delay != mrotateLast_delay){ //knob is rotating
    if (digitalRead(DELAY_PIN_DATA) != mrotate_delay) {  //switch A changed first -> rotating BRIGHTNESS_PIN_CLOCKwise
      doDelayLeft();
    } else {// switch B changed first -> rotating counterBRIGHTNESS_PIN_CLOCKwise
      doDelayRight();
    }
     
    print_delay();
  }
  mrotateLast_delay = mrotate_delay; 
  
  return brightness;
}

bool readswitch_brightness(){
  cli();  // disable / pause interrupts
  if(digitalRead(BRIGHTNESS_PIN_PUSH)!=0){ // switch is pressed
    while(digitalRead(BRIGHTNESS_PIN_PUSH)!=0){
    } //switch is currently pressed
    switchval_brightness = 1;
  } else {
    switchval_brightness = 0;
  } //switch is unpressed
  
  sei();  // enable interrupts 
  return switchval_brightness;    
}
 
bool readswitch_delay(){
  cli();  // disable / pause interrupts
  if(digitalRead(DELAY_PIN_PUSH)!=0){ // switch is pressed
    while(digitalRead(DELAY_PIN_PUSH)!=0){
    } //switch is currently pressed
    switchval_delay = 1;
  } else {
    switchval_delay = 0;
  } //switch is unpressed
  
  sei();  // enable interrupts 
  return switchval_delay;    
}


/**
 * FastLed functions
 */
void lightsOff() {
  fill_solid(leds1, NUM_LEDS_1, CRGB::Black);
  fill_solid(leds2, NUM_LEDS_2, CRGB::Black);
  FastLED.show();
}

void lightsOn() {
  //CHSV c = CHSV(0, 0, 0); // black
  //fill_solid(leds1, NUM_LEDS_1, c );

  fill_solid(leds1, NUM_LEDS_1, CRGB::Blue);
  fill_solid(leds2, NUM_LEDS_2, CRGB::Blue);
  FastLED.show();
}

void drawLine(CRGB leds[], int pos, int length) {
  int i;
  for (i = 0; i < length; i++) {
    if (pos+i <= NUM_LEDS_1) {
      if (i >= RED_NUM_FADE) {
        leds[pos+i] = CRGB::OrangeRed;  
      } else {
        leds[pos+i] = CRGB::Red;    
      }
      
    }
  }
  delay(delay_value * 1);
  FastLED.show();
}

void drawLineBackward(CRGB leds[], int pos, int length) {
  int i;
  for (i = 0; i < length; i++) {
    if (pos-i >= 0 && i <= RED_MAX) {
      if (i >= RED_NUM_FADE) {
        leds[pos-i] = CRGB::OrangeRed;  
      } else {
        leds[pos-i] = CRGB::Red;
      }
      
    }
  }
  delay(delay_value * 1);
  FastLED.show();
}

void drawKit() {
  int i;
  for (i = 0; i <= NUM_LEDS_1 - RED_NUM_MIN; i++) {
    drawLine(leds1, i, i);
    drawLine(leds2, i, i);
    leds1[i] = CRGB::Black;
    leds2[i] = CRGB::Black;
    FastLED.show();
  }
  delay(delay_value * 25);
  
  for (i = 0; i <= NUM_LEDS_1 - RED_NUM_MIN + 1; i++) {
    drawLineBackward(leds1, NUM_LEDS_1 - i, i);
    drawLineBackward(leds2, NUM_LEDS_1 - i, i);
    leds1[NUM_LEDS_1 - i] = CRGB::Black;
    leds2[NUM_LEDS_1 - i] = CRGB::Black;
    FastLED.show();
  }
  delay(delay_value * 25);
}



/**
 * the loop
 */
void loop() {   
  drawKit();
  
  // Turn the LED on, then pause
  digitalWrite(LED_BUILTIN, HIGH);   // turn the LED on (HIGH is the voltage level)
  delay(delay_value * 3);
  
  // Now turn the LED off, then pause
  digitalWrite(LED_BUILTIN, LOW);    // turn the LED off by making the voltage LOW
  delay(delay_value * 3);
}
