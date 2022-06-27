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

u8 display_state[BUTTON_COUNT] = {0};
u8 temp_state[BUTTON_COUNT] = {0};
u8 beat = 0;

u8 is_cell_alive(u8 x, u8 y)
{
  return x > 0 && x < 9 && y > 0 && y < 9 && display_state[x + y * 10];
}

u8 count_cell_neighbors(u8 x, u8 y)
{
  u8 count = 0;
  count += is_cell_alive(x - 1, y - 1);
  count += is_cell_alive(x, y - 1);
  count += is_cell_alive(x + 1, y - 1);
  count += is_cell_alive(x - 1, y);
  count += is_cell_alive(x + 1, y);
  count += is_cell_alive(x - 1, y + 1);
  count += is_cell_alive(x, y + 1);
  count += is_cell_alive(x + 1, y + 1);
  return count;
}

u8 does_cell_live(u8 x, u8 y)
{
  u8 alive = is_cell_alive(x, y);
  u8 neighbors = count_cell_neighbors(x, y);
  if (alive)
  {
    if (neighbors < 2)
    {
      return 0;
    }
    if (neighbors > 3)
    {
      return 0;
    }
  }
  else
  {
    if (neighbors == 3)
    {
      return 1;
    }
  }
  return alive;
}

//______________________________________________________________________________

void app_surface_event(u8 type, u8 index, u8 value)
{
  switch (type)
  {
  case TYPEPAD:
  {
    // toggle it and store it off, so we can save to flash if we want to
    if (value)
    {
      display_state[index] = MAXLED * !display_state[index];
    }

    // example - light / extinguish pad LEDs
    hal_plot_led(TYPEPAD, index, 0, 0, display_state[index]);
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
//   // example - send MIDI clock at 125bpm
#define TICK_MS 500

  static u16 ms = TICK_MS;
  //   static u8 tick = 0;
  if (++ms >= TICK_MS)
  {
    ms = 0;
    //     if (++tick <= BUTTON_COUNT)
    //     {
    //       hal_plot_led(TYPEPAD, tick, 0, 0, MAXLED);
    //     }
    //     // send a clock pulse up the USB
    //   }
    for (int i = 1; i < 9; ++i)
    {
      for (int j = 1; j < 9; ++j)
      {
        // u8 b = g_Buttons[j * 10 + i];

        temp_state[j * 10 + i] = does_cell_live(i, j);
        hal_plot_led(TYPEPAD, j * 10 + i, 0, 0, temp_state[j * 10 + i] * MAXLED);
      }
    }

    memcpy(display_state, temp_state, sizeof(display_state));
    beat = !beat;
    hal_plot_led(TYPEPAD, 1, 0, beat * MAXLED, 0);
  }
}

// u8 is_pad_index(u8 index)
// {
//   return index > 7;
// }

//______________________________________________________________________________

void app_init(const u16 *adc_raw)
{
  // example - load button statess from flash
  // hal_read_flash(0, g_Buttons, BsUTTON_COUNT);

  // example - light the LEDs to say hello !

  display_state[11] = 1;
  hal_plot_led(TYPEPAD, 11, 0, 0, MAXLED);
  display_state[12] = 1;
  hal_plot_led(TYPEPAD, 12, 0, 0, MAXLED);
  display_state[22] = 1;
  hal_plot_led(TYPEPAD, 22, 0, 0, MAXLED);
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
