import dgram from 'dgram';

const socket = dgram.createSocket('udp4');
const message = new Buffer.from("HELLO RUST");

function send() {
  socket.send(message, 0, message.length, 59999, 'localhost');
}

socket.on('message', (msg, rinfo) => {
  console.log(`MSG: ${msg.toString()},`, `ADDR: ${rinfo.address}:${rinfo.port}`);
})

setInterval(send, 5000);
send();