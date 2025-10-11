# Pi Dry
## A DIY Raspberry Pi based filament dryer. 

### BOM 
* Raspberry Pi 4B $60
* [Bestol Room Heater](https://www.amazon.com/gp/product/B081NQH1BD/ref=ox_sc_act_title_6?smid=A3MM3IVRRE6BVX&th=1) 12V 120W PTC heater w/ 12V fan attached $21
* [DHT22/AM2302](https://www.amazon.com/gp/product/B073F472JL/ref=ox_sc_act_title_5?smid=A2177J1RKY6IS3&psc=1) 2 Pack Temperature and Humidity sensor, 3.3v or 5v $13
* [Rotary knob with OLED](https://www.amazon.com/gp/product/B0DMYQHM9J/ref=ox_sc_act_title_4?smid=A1ASY00QOMN6RD&psc=1) I2C display, rotary and 2 buttons $10
* Power Devices $43
   * [IRLB3034 MOSFET](https://www.amazon.com/gp/product/B0CBKH7DGD/ref=ox_sc_act_title_1?smid=A3FX7C4A9P37IQ&th=1) 5pk for controlling heater and fan $10
   * [12V 20A Power Supply](https://www.amazon.com/gp/product/B078RTV41D/ref=ox_sc_act_title_3?smid=AA0YO4F2UD50F&th=1) 3 outputs so that Pi can be powered off the supply eventually $22
   * [NEMA 1-15 Plug SJT Cable](https://www.amazon.com/gp/product/B08MDV915C/ref=ox_sc_act_title_2?smid=AKX4PUL0YEZW2&th=1) For connecting power supply to wall $11
#### Total cost $147
#### Comparables:
* [SUNLU S4 $140]() - Better heater, 3 fans, touch screen
* [EIBOS $140](https://shop.eibos3d.com/products/pre-order-eibos-3d-filament-dryer-polyphemus?variant=42740222525619&country=US&currency=USD&utm_medium=product_sync&utm_source=google&utm_content=sag_organic&utm_campaign=sag_organic&srsltid=AfmBOoqEt9K-tem4avEpACNct-SyEMkT0bSgKzr72J4gzai6orsZAps2dJQ) - I really just think this one is overpriced
* [Creality Space Pi $95](https://www.microcenter.com/product/678579/Space_Pi_Plus_Filament_Dryer;__2_rolls_capacity?storeID=151) - Touch Screen, lower wattage heater, verry compact

### Phases
1. Get hardware communication working and expose a simple api/cli to interact with device
2. Add knob and screen functionality
3. Integrate the Matter protocol for smart home communication using the matter-rs library
4. Explore using better suited hardware for the use case. Pi is way too much compute for a project like this and the cost of the BOM makes this worthless as a real product.

### Code Overview
* Main Thread:
    * Controls the hardware
    * Sends and recieves messages to/from CLI and other threads
* Physical Input Thread:
    * Reads input from rotary knob
    * Displays received state from main thread on the screen
    * Sends received input to the main thread for processing
* Matter Input Thread:
    * Receives and Sends messages to Matter Hub via the matter-rs library
    * Receives and Sends messages to the main thread 
