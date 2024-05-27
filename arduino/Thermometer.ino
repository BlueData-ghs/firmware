// Gavin Palmer, May 2024

const int tempPin = A0; // analog pin 0

void setup()
{
  Serial.begin(9600);
  pinMode(tempPin, INPUT);
}

void loop()
{
  int tempReading = analogRead(tempPin);

  double tempK = log(10000.0 * ((1024.0 / tempReading - 1)));
  tempK = 1 / (0.001129148 + (0.000234125 + (0.0000000876741 * tempK * tempK )) * tempK ); // Temp Kelvin
  float tempC = tempK - 273.15; // Convert Kelvin to Celcius

  // Display Temperature in C
  //Serial.print("Temp: ");
  Serial.println(tempC);
  //Serial.println("C");

  delay(1000);
}
