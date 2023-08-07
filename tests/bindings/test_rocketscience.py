import asyncio

from rocketscience import *

command = Part(name = "Mk1 Command Pod", cost = 600, weight = 840)
tank = Part(name = "FL-T200", cost = 275, weight = 1125)
engine = Part(name = "LV-T30", cost = 1100, weight = 1250)

rocket = Rocket(name = "Orbiter")
rocket.add(part = command)
rocket.add(part = tank)
rocket.add(part = engine)

rocket.lock_steering(direction = Direction.DOWN)

try:
    rocket.launch()
    assert(not("Should have thrown a LaunchError exception!"))
except LaunchError.RocketLaunch:
    # It's okay!
    pass

rocket.lock_steering(direction = Direction.UP)
assert rocket.launch()

# We can launch the rocket asynchronously
rocket.lock_steering(direction = Direction.UP)
assert asyncio.run(launch_after(rocket, 20))
