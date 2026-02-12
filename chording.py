import keyboard
import time
import socket

pressed = set()
last_time = time.time()

waycrust_socket_path = "/tmp/waycrust.sock"

def send_socket_event(event):
    client = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    message = event + "\n"
    client.connect(waycrust_socket_path)
    client.sendall(message.encode())
    client.close()

def validate_chord():
    global pressed
    return all([key in "sdfjkl" for key in pressed])


def flush_chord():
    global pressed
    if pressed:
        if validate_chord():
            chord = "".join(sorted(pressed))
            match chord:
                case "fl":
                    send_socket_event("FLIP")
                case "fks":
                    send_socket_event("EXEC firefox")
                case "df":
                    send_socket_event("EXIT")
            print("Chord:", chord)
        else:
            print("Invalid chord")
        pressed = set()

def on_event(e):
    global last_time
    if e.event_type == "down":
        pressed.add(e.name)
        last_time = time.time()
    elif e.event_type == "up":
        if time.time() - last_time > 0.05:
            flush_chord()

keyboard.hook(on_event)
keyboard.wait()