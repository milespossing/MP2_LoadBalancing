import socket
import json
import subprocess

from flask import Flask

app = Flask(__name__)

@app.route('/', methods=['GET'])
def index():
    return socket.gethostbyname(socket.gethostname())

@app.route('/', methods=['POST'])
def post():
    subprocess.Popen(["python", "stress.py"])
    return ''

app.run(host='0.0.0.0', port=3030)

