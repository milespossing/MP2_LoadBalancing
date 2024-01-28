import datetime

with open('test.txt', 'w', encoding='utf-8') as f:
    dt = str(datetime.datetime.now())
    f.write(dt)

print('hello')

