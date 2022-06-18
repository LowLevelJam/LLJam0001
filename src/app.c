/******************************************************************************

 Copyright (c) 2015, Focusrite Audio Engineering Ltd.
 All rights reserved.

 Redistribution and use in source and binary forms, with or without
 modification, are permitted provided that the following conditions are met:

 * Redistributions of source code must retain the above copyright notice, this
 list of conditions and the following disclaimer.

 * Redistributions in binary form must reproduce the above copyright notice,
 this list of conditions and the following disclaimer in the documentation
 and/or other materials provided with the distribution.

 * Neither the name of Focusrite Audio Engineering Ltd., nor the names of its
 contributors may be used to endorse or promote products derived from
 this software without specific prior written permission.

 THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

 *****************************************************************************/

//______________________________________________________________________________
//
// Headers
//______________________________________________________________________________

#include "app.h"

//______________________________________________________________________________
//
// This is where the fun is!  Add your code to the callbacks below to define how
// your app behaves.
//
// In this example, we either render the raw ADC data as LED rainbows or store
// and recall the pad state from flash.
//______________________________________________________________________________

// store ADC frame pointer
// static const u16 *g_ADC = 0;

// buffer to store pad states for flash save
#define BUTTON_COUNT 100

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

u8 is_input_button(u8 index)
{
  // TODO: check if this is an input button
  return index == 10;
}

void app_surface_event(u8 type, u8 index, u8 value)
{
  switch (type)
  {
  case TYPEPAD:
  {
    if (is_input_button(index))
    {
      if (value)
      {
        held_button_index = index;
      }
      else
      {
        held_button_index = 0;
      }
    }

    if (held_button_index)
    {
      display_state[index].input_index = held_button_index;
      display_state[index].type = INPUT;
    }

    // example - light / extinguish pad LEDs
    // hal_plot_led(TYPEPAD, index, 0, 0, display_state[index].output * MAXLED);
    break;
  }
  }
}

//______________________________________________________________________________

void app_midi_event(u8 port, u8 status, u8 d1, u8 d2) {}

//______________________________________________________________________________

void app_sysex_event(u8 port, u8 *data, u16 count) {}

//______________________________________________________________________________

void app_aftertouch_event(u8 index, u8 value) {}

//______________________________________________________________________________

void app_cable_event(u8 type, u8 value) {}

//______________________________________________________________________________

void app_timer_event()
{
#define TICK_MS 500

  static u16 ms = TICK_MS;
  if (++ms >= TICK_MS)
  {
    ms = 0;
    display_state[10].output = !display_state[10].output;

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
            break;
          case OR:
            display_state[index].output = a || b;
            break;
          case XOR:
            display_state[index].output = a ^ b;
            break;
          case NOT:
            display_state[index].output = !a;
            break;
          }
          last_row_value = display_state[index].output;
        }
        hal_plot_led(TYPEPAD, index, 0, 0, display_state[index].output * MAXLED);
      }
      display_state[y * 10 + 9].output = last_row_value;
      hal_plot_led(TYPEPAD, y * 10 + 9, 0, 0, display_state[y * 10 + 9].output * MAXLED);
    }
    hal_plot_led(TYPEPAD, 10, 0, 0, display_state[10].output * MAXLED);
    // {
    //   if (display_state[i].type == INPUT)
    //   {
    //     display_state[i].output = display_state[display_state[i].input_index].output;
    //   }
    //   else if (display_state[i].type == GATE)
    //   {
    //     u8 a = display_state[display_state[i].gate_input_index_a].output;
    //     u8 b = display_state[display_state[i].gate_input_index_b].output;
    //     u8 output = 0;
    //     switch (display_state[i].gate_type)
    //     {
    //     case AND:
    //       output = a & b;
    //       break;
    //     case OR:
    //       output = a | b;
    //       break;
    //     case XOR:
    //       output = a ^ b;
    //       break;
    //     case NOT:
    //       output = !a;
    //       break;
    //     }
    //     display_state[i].output = output;
    //   }

    //   hal_plot_led(TYPEPAD, i, 0, 0, display_state[i].output * MAXLED);
    // }

    // hal_plot_led(TYPEPAD, 10, 0, display_state[10].output * MAXLED, 0);
  }
}

//______________________________________________________________________________

void app_init(const u16 *adc_raw)
{
  // example - load button statess from flash
  // hal_read_flash(0, g_Buttons, BsUTTON_COUNT);

  hal_plot_led(TYPEPAD, 91, MAXLED, 0, MAXLED);
  hal_plot_led(TYPEPAD, 92, 0, MAXLED, 0);
  hal_plot_led(TYPEPAD, 93, MAXLED, 0, 0);
  hal_plot_led(TYPEPAD, 94, 0, MAXLED, MAXLED);

  // example - light the LEDs to say hello !

  // for (int i = 1; i < 9; ++i)
  // {
  //   for (int j = 1; j < 9; ++j)
  //   {
  //     // u8 b = g_Buttons[j * 10 + i];

  //     temp_state[j * 10 + i] = does_cell_live(i, j);
  //     hal_plot_led(TYPEPAD, j * 10 + i, 0, 0, temp_state[j * 10 + i] * MAXLED);
  //   }
  // }

  // memcpy(display_state, temp_state, sizeof(display_state));

  // store off the raw ADC frame pointer for later use
  // g_ADC = adc_raw;
}
