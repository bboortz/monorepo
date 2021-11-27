#include <Bounce2.h>
#include "DigiKeyboard.h"


// define buttons
#define BUTTON1_PIN 0 
#define BUTTON2_PIN 1
#define BUTTON3_PIN 2
#define DEBOUNCE_INTERVAL 15

// instanciate button objects
Bounce2::Button button1 = Bounce2::Button();
Bounce2::Button button2 = Bounce2::Button();
Bounce2::Button button3 = Bounce2::Button();


void setup() {

  // setup buttons
  button1.attach( BUTTON1_PIN, INPUT );
  button1.interval(DEBOUNCE_INTERVAL); 
  button1.setPressedState(LOW);
  
  button2.attach( BUTTON2_PIN, INPUT );
  button2.interval(DEBOUNCE_INTERVAL);
  button2.setPressedState(LOW);
  
  button3.attach( BUTTON3_PIN, INPUT ); 
  button3.interval(DEBOUNCE_INTERVAL);
  button3.setPressedState(LOW); 
}

void loop() {
  // update the buttons
  button1.update();
  button2.update();
  button3.update();

  // check pressed buttons
  if ( button1.pressed() ) {
    DigiKeyboard.print("PASSWORD\n");
  }
  if ( button2.pressed() ) {
    DigiKeyboard.print("2P");
    // DigiKeyboard.sendKeyStroke(KEY_PrintScreen); 
  }
  if ( button3.pressed() ) {
    DigiKeyboard.sendKeyStroke(KEY_M , MOD_CONTROL_LEFT | MOD_SHIFT_LEFT); 
  }
}
