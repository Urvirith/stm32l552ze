# Make Script For Embedded Rust
TOOLCHAIN	:= arm-none-eabi-
CC		    := $(TOOLCHAIN)gcc		# C Compiler
AS			:= $(TOOLCHAIN)as		# Assembler
LD 			:= $(TOOLCHAIN)ld 		# Linker
OBJ 		:= $(TOOLCHAIN)objcopy	# Object Copy

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
BIN_DIR 	:= ./bin

#ONLY ONE
STARTUP		:= startup_ARMCM33.s

#ONLY ONE
LINKER		:= gcc_arm.ld

release: $(BIN_DIR)/main.bin

# Build An ELF 
$(BIN_DIR)/main.bin: $(BIN_DIR)/main.elf
	$(OBJ) $(OBJFLAGS) $^ $@

# Build An ELF 
$(BIN_DIR)/main.elf: $(LINK_DIR)/$(LINKER) $(BIN_DIR)/main.o $(BIN_DIR)/startup.o
	$(LD) -Os -s $(LDFLAGS) $^ -o $@

# Build Dependances
$(BIN_DIR)/startup.o: $(START_DIR)/$(STARTUP)
	$(AS) $< $(ASFLAGS) -o $@

# Build The Rust Project, .cargo and Cargo.Toml hold the flags for this
$(BIN_DIR)/main.o:
	cargo build --release

# Clean The Build Folder To Allow For A Complete Rebuild
clean:
	rm -f $(BIN_DIR)/*.o
	rm -f $(BIN_DIR)/*.elf
	rm -f $(BIN_DIR)/*.bin
	cargo clean

# Flash the board us STMProgrammer Utility
flash:
	STM32_Programmer_CLI -c port=SWD -w $(BIN_DIR)/main.bin 0x08000000

info:
	STM32_Programmer_CLI -c port=SWD

reset:
	STM32_Programmer_CLI -c port=SWD -rst

hard_reset:
	STM32_Programmer_CLI -c port=SWD -hardRst

setup:
	mkdir bin