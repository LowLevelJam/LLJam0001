

#include "app.h"

// buffer to store pad states for flash save
#define BUTTON_COUNT 100
#define CLOCK_INDEX 10
#define AND_INDEX 91
#define OR_INDEX 92
#define XOR_INDEX 93
#define NOT_INDEX 94
#define NAND_INDEX 95
#define NOR_INDEX 96

enum CELL_TYPE
{
  EMPTY = 0,
  INPUT = 1,
  GATE = 2,
};

enum GATE_TYPE
{
  AND = 0,
  OR = 1,
  XOR = 2,
  NOT = 3,
  NAND = 4,
  NOR = 5,
};

struct cell
{
  enum CELL_TYPE type;
  u8 input_index;
  enum GATE_TYPE gate_type;
  u8 gate_input_index_a;
  u8 gate_input_index_b;
  u8 output;
};

struct cell display_state[BUTTON_COUNT] = {0};
// u8 temp_state[BUTTON_COUNT] = {0};
u8 held_button_index = 0;

//______________________________________________________________________________

// This function is kindof insane.
// It just checks if the supplied index is one of the allowed side buttons
// For the user input, clocks, gates, and output routing.
// This could be compacted but it works fine and I don't feel like changing it. c:
u8 is_input_button(u8 index)
{
  return (index > 90 && index < 97) || (index > 0 && index < 11) || index == 19 || index == 29 || index == 39 || index == 49 || index == 59 || index == 69 || index == 79 || index == 89;
}

// user input buttons range from 1 to 8
// These are the bottom row of buttons
u8 is_user_input_button(u8 index)
{
  return index > 0 && index < 9;
}

// This is the callback that's triggered when a button is pressed
void app_surface_event(u8 type, u8 index, u8 value)
{
  // The switch here could just be an if because we don't care about the other case just yet
  // But it was here from the example code so I kept it
  switch (type)
  {
  case TYPEPAD:
  {
    // Here's where that insane function from before comes in
    if (is_input_button(index))
    {
      // We check if value to know if the button is being pressed or released
      // Value will be 0 if it's released and some positive number if it's being pressed
      if (value)
      {
        held_button_index = index;
      }
      else
      {
        held_button_index = 0;
      }
      if (is_user_input_button(index))
      {
        // display_state[index].type = INPUT;
        // display_state[index].input_index = index;
        display_state[index].output = value ? 1 : 0;
      }
      return;
    }
    // anding with value ensures this only triggers on a down press
    if (held_button_index && value)
    {
      // If the button already has the same input index, we want to remove it
      // This is the only way to 'unset' a button
      if (display_state[index].input_index == held_button_index)
      {
        display_state[index].input_index = 0;
        display_state[index].type = EMPTY;
        hal_plot_led(TYPEPAD, index, MAXLED, 0, 0);
      }
      else
      {
        // Check for the gate type
        // This could be done with a lookup table later
        switch (held_button_index)
        {

        case AND_INDEX:
        {
          display_state[index].input_index = held_button_index;
          display_state[index].gate_type = AND;
          display_state[index].type = GATE;
          display_state[index].gate_input_index_a = index - 1;
          display_state[index].gate_input_index_b = index - 2;
          hal_plot_led(TYPEPAD, index, 0, 0, MAXLED);
          break;
        }
        case OR_INDEX:
        {
          display_state[index].input_index = held_button_index;
          display_state[index].gate_type = OR;
          display_state[index].type = GATE;
          display_state[index].gate_input_index_a = index - 1;
          display_state[index].gate_input_index_b = index - 2;
          hal_plot_led(TYPEPAD, index, 0, MAXLED, 0);
          break;
        }
        case XOR_INDEX:
        {
          display_state[index].input_index = held_button_index;
          display_state[index].gate_type = XOR;
          display_state[index].type = GATE;
          display_state[index].gate_input_index_a = index - 1;
          display_state[index].gate_input_index_b = index - 2;
          hal_plot_led(TYPEPAD, index, MAXLED, 0, 0);
          break;
        }
        case NOT_INDEX:
        {
          display_state[index].input_index = held_button_index;
          display_state[index].gate_type = NOT;
          display_state[index].type = GATE;
          display_state[index].gate_input_index_a = index - 1;
          hal_plot_led(TYPEPAD, index, 0, 0, MAXLED);
          break;
        }
        case NAND_INDEX:
        {
          display_state[index].input_index = held_button_index;
          display_state[index].gate_type = NAND;
          display_state[index].type = GATE;
          display_state[index].gate_input_index_a = index - 1;
          display_state[index].gate_input_index_b = index - 2;
          hal_plot_led(TYPEPAD, index, MAXLED / 2, 0, MAXLED / 2);
          break;
        }
        case NOR_INDEX:
        {
          display_state[index].input_index = held_button_index;
          display_state[index].gate_type = NOR;
          display_state[index].type = GATE;
          display_state[index].gate_input_index_a = index - 1;
          display_state[index].gate_input_index_b = index - 2;
          hal_plot_led(TYPEPAD, index, 0, MAXLED / 2, 0);
          break;
        }
        default:
        {
          // This case handles both user input and the clock
          // Since both have their output set external to the simulation
          // They can be treated as input targets
          display_state[index].input_index = held_button_index;
          display_state[index].type = INPUT;
          hal_plot_led(TYPEPAD, index, 0, MAXLED, 0);
          break;
        }
        }
      }
    }
    break;
  }
  }
}

// None of these callbacks are used
// But the compiler complains if the stubs aren't here
//______________________________________________________________________________

void app_midi_event(u8 port, u8 status, u8 d1, u8 d2) {}

//______________________________________________________________________________

void app_sysex_event(u8 port, u8 *data, u16 count) {}

//______________________________________________________________________________

void app_aftertouch_event(u8 index, u8 value) {}

//______________________________________________________________________________

void app_cable_event(u8 type, u8 value) {}

//______________________________________________________________________________

// This is the main loop of the application
// The documentation would have me believe it's run every millisecond
// Though I don't know how accurate the timing is
// This is where the simulation is done
// And where the lights are updated
void app_timer_event()
{
#define TICK_MS 500

  static u16 ms = TICK_MS;
  if (++ms >= TICK_MS)
  {
    ms = 0;
    display_state[10].output = !display_state[10].output;
  }
  // for (u8 i = 0; i < BUTTON_COUNT - 10; i++)
  for (u8 y = 1; y < 9; y++)
  {
    u8 last_row_value = 0;
    for (u8 x = 1; x < 9; x++)
    {
      u8 index = x + y * 10;
      if (display_state[index].type == INPUT)
      {
        display_state[index].output = display_state[display_state[index].input_index].output;
        last_row_value = display_state[index].output;
      }
      else if (display_state[index].type == GATE)
      {
        u8 a = display_state[display_state[index].gate_input_index_a].output;
        u8 b = display_state[display_state[index].gate_input_index_b].output;
        switch (display_state[index].gate_type)
        {
        case AND:
          display_state[index].output = a && b;
          hal_plot_led(TYPEPAD, index, MAXLED, 0, MAXLED);
          break;
        case OR:
          display_state[index].output = a || b;
          hal_plot_led(TYPEPAD, index, 0, MAXLED, 0);
          break;
        case XOR:
          display_state[index].output = a ^ b;
          hal_plot_led(TYPEPAD, index, MAXLED, 0, 0);
          break;
        case NOT:
          display_state[index].output = !a;
          hal_plot_led(TYPEPAD, index, 0, MAXLED, MAXLED);
          break;
        case NAND:
          display_state[index].output = !(a && b);
          hal_plot_led(TYPEPAD, index, MAXLED, MAXLED, 0);
          break;
        case NOR:
          display_state[index].output = !(a || b);
          hal_plot_led(TYPEPAD, index, MAXLED, MAXLED, MAXLED);
          break;
        }
        last_row_value = display_state[index].output;
        continue;
      }
      else if (display_state[index].type == EMPTY)
      {
        display_state[index].output = 0;
      }
      hal_plot_led(TYPEPAD, index, 0, 0, display_state[index].output * MAXLED);
    }
    display_state[y * 10 + 9].output = last_row_value;
    hal_plot_led(TYPEPAD, y * 10 + 9, 0, 0, display_state[y * 10 + 9].output * MAXLED);
  }
  hal_plot_led(TYPEPAD, 10, 0, 0, display_state[10].output * MAXLED);
}

//______________________________________________________________________________

void app_init(const u16 *adc_raw)
{
  // This just sets the color coding for the gate select buttons at the top.
  // The LEDs are static so these values don't need to be refreshed
  hal_plot_led(TYPEPAD, 91, MAXLED, 0, MAXLED);
  hal_plot_led(TYPEPAD, 92, 0, MAXLED, 0);
  hal_plot_led(TYPEPAD, 93, MAXLED, 0, 0);
  hal_plot_led(TYPEPAD, 94, 0, MAXLED, MAXLED);
  hal_plot_led(TYPEPAD, 95, MAXLED, MAXLED, 0);
  hal_plot_led(TYPEPAD, 96, MAXLED, MAXLED, MAXLED);
}
