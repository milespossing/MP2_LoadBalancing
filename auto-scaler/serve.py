import socket
import json
import subprocess

from flask import Flask

app = Flask(__name__)

@app.route('/', methods=['GET'])
def index():
    return socket.gethostname()

@app.route('/', methods=['POST'])
def post():
    subprocess.Popen(["python", "stress.py"])
    return ''

app.run()

