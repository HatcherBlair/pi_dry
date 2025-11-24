# Pi Dry
## A DIY Raspberry Pi based filament dryer. 

### Video Link
[Pi Dry Demo](https://youtu.be/kvUcSSZsg2s). Sorry about the audio quality when the fan is running. I thought my phone would be better at noise rejection. 

### BOM 
* Raspberry Pi 4B $60
* [Bestol Room Heater](https://www.amazon.com/gp/product/B081NQH1BD/ref=ox_sc_act_title_6?smid=A3MM3IVRRE6BVX&th=1) 12V 120W PTC heater w/ 12V fan attached $21
* [DHT22/AM2302](https://www.amazon.com/gp/product/B073F472JL/ref=ox_sc_act_title_5?smid=A2177J1RKY6IS3&psc=1) 2 Pack Temperature and Humidity sensor, 3.3v or 5v $13
* [FreeNove I2C LCD](https://www.amazon.com/dp/B0B76Z83Y4?ref=ppx_yo2ov_dt_b_fed_asin_title) Replaced the below display $9.56
* [Rotary knob with OLED](https://www.amazon.com/gp/product/B0DMYQHM9J/ref=ox_sc_act_title_4?smid=A1ASY00QOMN6RD&psc=1) I2C display, rotary and 2 buttons $10
* Power Devices $39.79
   * [SunFounder 2 Channel DC 5V Relay Module](amazon.com/dp/B00E0NTPP4?ref=ppx_yo2ov_dt_b_fed_asin_title) Relay Module for controlling the fan and heater $6.79
   * [12V 20A Power Supply](https://www.amazon.com/gp/product/B078RTV41D/ref=ox_sc_act_title_3?smid=AA0YO4F2UD50F&th=1) 3 outputs so that Pi can be powered off the supply eventually $22
   * [NEMA 1-15 Plug SJT Cable](https://www.amazon.com/gp/product/B08MDV915C/ref=ox_sc_act_title_2?smid=AKX4PUL0YEZW2&th=1) For connecting power supply to wall $11
#### Total cost $153.35
#### Comparables:
* [SUNLU S4 $140]() - Better heater, 3 fans, touch screen
* [EIBOS $140](https://shop.eibos3d.com/products/pre-order-eibos-3d-filament-dryer-polyphemus?variant=42740222525619&country=US&currency=USD&utm_medium=product_sync&utm_source=google&utm_content=sag_organic&utm_campaign=sag_organic&srsltid=AfmBOoqEt9K-tem4avEpACNct-SyEMkT0bSgKzr72J4gzai6orsZAps2dJQ) - I really just think this one is overpriced
* [Creality Space Pi $95](https://www.microcenter.com/product/678579/Space_Pi_Plus_Filament_Dryer;__2_rolls_capacity?storeID=151) - Touch Screen, lower wattage heater, verry compact

### Usage
This project is currently not implemented to be flexible with wiring configuration. For the project to work you must connect the devices using the following pins:
* Fan: Pin 8
* Heater: Pin 10
* Back Button: Pin 11
* Confirm Button: Pin 13
* Right: Pin 19
* Left: Pin 21
* I2C: Pins 3 and 5

Start the application from the command line and then use the buttons and rotary wheel to interact with the system. Back will take you to the list of materials, use the wheel to move left and right through the list. Press confirm to select that material and the heater will target that temperature for that duration. 

### What's next
Currently, the project is in a very basic state, the base functionality is there but it is not polished. The next step for me is going to be to rework the state object and the updating logic. The primary objective of this is to rework the display. Currently, I draw the entire display once per second. This can cause interacting with the device to feel unresponsive and it also wastes a lot of time on the I2C bus. The bus isn't shared across threads and so it is not a major concern but it is unnecessary to be sending that much data over the bus. The goal would be to only write the diff of the display when there is a change. That would be when the temperature, humidity, or timer changes and when scrolling through the list of materials.

Once I am happy with the local functionality the goal is to add matter support. I am not sure how feature complete the device will be on matter. I don't like the idea of being able to press a button that turns on a 120W heater somewhere else. Matter might provide status updates only. 

The next goal is to revise the hardware. The hardware was selected for being easy to wire together and experiment with. My goal is to make the next iteration much more compact and to power the Pi with the power supply used for the heater and fan. This will need additional dedicated circuitry to step the voltage down from 12v to 5v. I also want to consider moving away from the Pi. The Pi is way more compute than is needed for the project and there are ESP32 boards that support the matter protocol.

### Code Overview:
#### Dryer Module
The main module that controls everything. This module contains the state that is shared across all threads and the update function that drives the state. 

#### Button Cluster Module
This is just a container for the 4 input buttons. These were pulled out from the dryer module because they each have an asynchronous callback. The values are also never read from these pins, so they can be nested in the state and ignored. They will lose their callback when dropped so the state object must hold on to them for the lifetime of the application.

#### Display Module
This just stores the state of the display, will likely be removed when I rewrite the display interface.
Working on re-write in display branch. The new function diffs the internal display state with the text that wants to be displayed and only writes the diff. This function hasn't been tested as I am not near my Pi and can only remotely compile for syntax issues and the likes. 

#### LCD Interface Module
Stateless helper functions for driving the display. The display is a little difficult to work with...(although not nearly as difficult as the original OLED). There are two chips on the board, one that drives the display and one that expands the I2C bus into 8bit commands. The display runs in 4-bit mode with a RS R/W EN and BL bit. RS specifies Data/Command, R/W is the read/write bit, EN is the enable line (more on this later), and BL is the backlight. The control bits must be sent with every command. Sending a one-byte command requires sending the high nibble followed by the low nibble. Writing a nibble to the display requires toggling the enable line. The display chip writes data to memory on the falling edge of the enable. So, to write a nibble, you send the data with EN high. Then you send the exact same data again with EN low. This stores the data on the first write. And drops the enable line to write the data to memory on the second write.

#### Shared Data Module
This is just a struct that stores all data that can be accessed from a Mutex. The pins all use asynchronus callback functions which are called from their own thread.

#### Temp Sensor Module
Reads temperature and humidity from the sensors. Also checks the CRC to ensure data wasn't corrupted in flight.

### Reflection:
I am relatively happy with the state that the project is in currently. I pushed all the core features of the dryer and delivered a functional filament dryer. That being said, it is just the basic functionality and there are more features I would have liked to have. Most of my wants are in the what's next section so I wont repeat that here. Like all projects, I wish I had more time. Although, I didn't necesarily make great use of the given time. I spent a good chunk of time up front selecting hardware and then slowly tested it in small modules getting each component validated. I wish that I had the intellegence to put each of my test programs into the bin directory and make them their own executable. This not only would have demonstrated the work I had been doing but also given me a place to easily expirement with different features as I integrated everything.

My experience with Rust was overall enjoyable. I had some issues with the RustAnalyzer being slow and not very helpful at times. After looking online, it seems that I am not alone. I found that it would fail silently and then not report my issues further down in the file until I fixed the hidden issue. I was excited about using a language with all the tooling built in and integrated but it seems that the language might be moving too fast for the tooling to get the love it deserves. Aside from my tooling gripes, the langauge was easier than expected to work with. The code that I wrote is relatively simple and I did run into some language skill issues in my Display branch, but overall I found myself able to write code relatively quickly. I also felt that I needed to spend less time fixing small mistakes compared to working in other languages because the small mistakes don't happen in Rust. 

Working with error handling was an area that I did not enjoy very much as well. Given that I am working with hardware, almost every operation can fail and error. A lot of these errors are not something that I am equiped to recover from, which made the experience less enjoyable. Sections of the code are just littered with try operators or calls to unwrap, and so many functions just return a Result with an empty Ok and a boxed error. I like the thought of being forced to handle errors where they occur and not using exceptions but I found myself just passing the errors around and not handling anything properly anyways.

I'm not sure how much Rust is in my future but I am glad to have taken this course and written this project in Rust. I plan to keep developing this project and turn it into a product that I regularly use but I expect that to be the extent of my Rust. I like the concepts of the language overall but I find it a bit restricting. I am sure that is a feeling that would reduce over time but I really like the freedom provided with a less rigid type system. 
