MAC  = yaneury@macbook.local
PORT = /dev/cu.usbmodem1301
ELF  = target/avr-none/release/baby-rover.elf

flash:
	cargo build --release
	avr-objcopy -O ihex $(ELF) target/firmware.hex
	rsync -a target/firmware.hex $(MAC):/Users/yaneury/Tmp/ArduinoOut
	ssh $(MAC) "/opt/homebrew/bin/avrdude -patmega328p -carduino -P$(PORT) -b115200 -D -Uflash:w:/Users/yaneury/Tmp/ArduinoOut/firmware.hex:i"
