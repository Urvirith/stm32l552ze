# Make Script For Embedded Rust
AS			:= arm-none-eabi-as			# assembler
LD 			:= arm-none-eabi-ld 		# linker
OBJ 		:= arm-none-eabi-objcopy	# Object Copy

# -Os				Optimize for Size
# -mcpu=cortex-m33	Compile for the ARM M33 Processor
# mthumb			Target the MTHUMB Instruction Set
ASFLAGS		:= -mcpu=cortex-m33 -mthumb
LDFLAGS 	:= -T 
OBJFLAGS	:= -O binary

#	EXAMPLE OF AUTOMATIC VARIABLES
#	%.o: %.c %.h common.h
#		$(CC) $(CFLAGS) -c $<
#
#	$@ Looks at the target
#	(Target)
#	%.o: 			%.c %.h common.h
#	
#	$< Looks at the first source
#			(First Source)
#	%.o: 	%.c 					%.h common.h
#		$(CC) $(CFLAGS) -c $<
#	$^ Looks at the first source
#			(All Source)
#	%.o: 	%.c %.h common.h
#		$(CC) $(CFLAGS) -c $^

START_DIR	:= ./src/startup
LINK_DIR 	:= ./src/linker
BLD_DIR 	:= ./build

#ONLY ONE
STARTUP		:= startup_ARMCM33.s

#ONLY ONE
LINKER		:= gcc_arm.ld

release: $(BLD_DIR)/main.bin

# Build An ELF 
$(BLD_DIR)/main.bin: $(BLD_DIR)/main.elf
	$(OBJ) $(OBJFLAGS) $^ $@

# Build An ELF 
$(BLD_DIR)/main.elf: $(LINK_DIR)/$(LINKER) $(BLD_DIR)/main.o $(BLD_DIR)/startup.o
	$(LD) -Os -s $(LDFLAGS) $^ -o $@

# Build Dependances
$(BLD_DIR)/startup.o: $(START_DIR)/$(STARTUP)
	$(AS) $< $(ASFLAGS) -o $@

# Build The Rust Project, .cargo and Cargo.Toml hold the flags for this
$(BLD_DIR)/main.o:
	cargo build --release

# Clean The Build Folder To Allow For A Complete Rebuild
clean:
	rm -f $(BLD_DIR)/*.o
	rm -f $(BLD_DIR)/*.elf
	rm -f $(BLD_DIR)/*.bin
	cargo clean

# Flash the board us STMProgrammer Utility
flash:
	STM32_Programmer_CLI -c port=SWD -w $(BLD_DIR)/main.bin 0x08000000

info:
	STM32_Programmer_CLI -c port=SWD

reset:
	STM32_Programmer_CLI -c port=SWD -rst

hard_reset:
	STM32_Programmer_CLI -c port=SWD -hardRst

setup:
	mkdir build