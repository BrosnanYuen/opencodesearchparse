global_var = 42

def add(a, b):
    return a + b

class Car:
    def __init__(self, brand, model):
        self.brand = brand
        self.model = model
        self.engine = self.Engine()
    def drive(self):
        if self.engine.status == "Running":
            print(f"Driving the {self.brand} {self.model}")
        else:
            print("Start the engine first!")
    class Engine:
        def __init__(self):
            self.status = "Off"
        def start(self):
            self.status = "Running"
            print("Engine started")
        def stop(self):
            self.status = "Off"
            print("Engine stopped")

PI = 3.14