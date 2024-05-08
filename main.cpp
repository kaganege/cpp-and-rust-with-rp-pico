#include "pico/stdlib.h"
#include "pico/cyw43_arch.h"
#include "macros.h"
#include "rusty.h"

int main()
{
  stdio_init_all();

  // Initialise the Wi-Fi chip
  if (cyw43_arch_init())
  {
    println("Wi-Fi init failed");
    return -1;
  }

  // Example to turn on the Pico W LED
  cyw43_arch_gpio_put(CYW43_WL_GPIO_LED_PIN, 1);

  while (true)
  {
    hello_from_rust();
    sleep_ms(1000);
  }
}
