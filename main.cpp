#include "pico/stdlib.h"
{% if board_type == "Pico W" %}
#include "pico/cyw43_arch.h"
{% endif %}
#include "macros.h"
#include "rusty.h"

int main()
{
  stdio_init_all();
  {% if board_type == "Pico" %}
  gpio_init(PICO_DEFAULT_LED_PIN);
  gpio_set_dir(PICO_DEFAULT_LED_PIN, GPIO_OUT);
  {% else %}
  // Initialise the Wi-Fi chip
  if (cyw43_arch_init())
  {
    println("Wi-Fi init failed");
    return -1;
  }
  {% endif %}

  while (true)
  {
    {% if board_type == "Pico" %}
    gpio_put(PICO_DEFAULT_LED_PIN, true);
    {% else %}
    cyw43_arch_gpio_put(CYW43_WL_GPIO_LED_PIN, true);
    {% endif %}
    hello_from_rust();
    sleep_ms(500);
    {% if board_type == "Pico" %}
    gpio_put(PICO_DEFAULT_LED_PIN, false);
    {% else %}
    cyw43_arch_gpio_put(CYW43_WL_GPIO_LED_PIN, false);
    {% endif %}
    hello_from_rust();
    sleep_ms(500);
  }
}
