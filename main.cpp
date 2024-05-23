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

  while (true)
  {
    cyw43_arch_gpio_put(CYW43_WL_GPIO_LED_PIN, true);
    hello_from_rust();
    sleep_ms(500);
    cyw43_arch_gpio_put(CYW43_WL_GPIO_LED_PIN, false);
    hello_from_rust();
    sleep_ms(500);
  }
}
