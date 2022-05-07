import flask
# import time

app = flask.Flask(__name__)


@app.route('/', methods=['POST'])
def index():
    data = flask.request.data
    """
    with open(f"/tmp/{time.time()}.jpg", 'wb') as f:
        print(f)
        r = f.write(data)
        print(r)
    """

    return f"size:  {len(data)}"


app.run(host='0.0.0.0', port=8000)
