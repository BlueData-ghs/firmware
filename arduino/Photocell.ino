// Gavin Palmer, May 2024

const int lightPin = A0; // analog pin 0
float initialReading;

void setup()
{
  Serial.begin(9600);
  pinMode(lightPin, INPUT);

  Serial.println("Collecting initial brightness reading...");
  initialReading = analogRead(lightPin);

  /* Collect and average values to use as the initial value

  for (int i)
  */
}

void loop() 
{
  float lightReading  = analogRead(lightPin);
  float percent = (lightReading / initialReading);
  Serial.print(percent * 100);
  Serial.println("%");
  delay(1000);
}
