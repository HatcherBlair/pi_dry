Pi Dry - A DIY Raspberry Pi based filament dryer. 

BOM - 
Raspberry Pi 4B, it's what I had on hand
PTC Heater
DC Fan
Temperature Sensor
Humidity Sensor
Rotary knob with button press
Simple display for displaying temperature and humidity information

Phases -
1) Get hardware communication working and expose a simple api/cli to interact with device
2) Add knob and screen functionality
3) Integrate the Matter protocol for smart home communication using the matter-rs library
4) Explore using better suited hardware for the use case. Pi is way too much compute for a project like this and the cost of the BOM makes this worthless as a real product.

Code Overview - 
a) Main Thread:
    * Controls the hardware
    * Sends and recieves messages to/from CLI and other threads
b) Physical Input Thread:
    * Reads input from rotary knob
    * Displays received state from main thread on the screen
    * Sends received input to the main thread for processing
c) Matter Input Thread:
    * Receives and Sends messages to Matter Hub via the matter-rs library
    * Receives and Sends messages to the main thread 
