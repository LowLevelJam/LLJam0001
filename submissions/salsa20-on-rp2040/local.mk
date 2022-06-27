CC			= gcc
MKDIR		= mkdir -p
RM			= rm -rf

INC_DIR		= inc
SRC_DIR		= src
STUB_DIR	= stub
OBJ_DIR		= obj
BUILD_DIR	= local_build

IFLAGS =
CFLAGS = -I$(INC_DIR) -g -DLOCAL_DEBUG

INC_FILES += $(INC_DIR)/salsa20.h
INC_FILES += $(INC_DIR)/common.h

SRC_FILES += $(SRC_DIR)/salsa20.c
SRC_FILES += $(STUB_DIR)/main.c

OBJ_FILES := $(patsubst $(SRC_DIR)/%, $(OBJ_DIR)/%, $(patsubst %.c, %.o, $(SRC_FILES)))

OTHER_DEPS=$(INC_DIR)/*.h Makefile

EXE=salsa20-local

.PHONY: all clean test

all: $(BUILD_DIR)/$(EXE)

$(BUILD_DIR)/$(EXE): $(OBJ_FILES)
	$(MKDIR) $(BUILD_DIR)
	$(CC) $^ -o $@

$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c $(OTHER_DEPS)
	$(MKDIR) $(OBJ_DIR)
	$(CC) $(CFLAGS) $(IFLAGS) -c $< -o $@

clean:
	$(RM) $(OBJ_DIR)/* $(BUILD_DIR)/*

test: $(BUILD_DIR)/$(EXE)
	$(BUILD_DIR)/$(EXE) -t