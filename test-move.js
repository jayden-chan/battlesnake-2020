const got = require('got');

const file = process.argv[2];
const turn = Number(process.argv[3]);

if (file === undefined || Number.isNaN(turn)) {
  console.log('Usage: node test-move.js <archive>.txt <move>');
  process.exit(1);
}

const game = require('fs')
  .readFileSync(file)
  .toString()
  .split('\n')
  .filter(e => e != "");

(async () => {
  try {
    const {body} = await got.post('http://localhost:9000/start', {
      body: game[turn - 10],
      responseType: 'json'
    });
  } catch (error) {
    console.log(error);
  }

  for (let i = -10; i <= 0; i++) {
    try {
      const {body} = await got.post('http://localhost:9000/move', {
        body: game[turn + 1 + i],
        responseType: 'json'
      });
      console.log(`turn: ${turn + i} move: ${JSON.parse(body).move}`);
    } catch (error) {
      console.log('ERROR');
      console.log(error);
    }

    await new Promise(resolve => setTimeout(resolve, 100));
  }

  await got.post('http://localhost:9000/end', {
    body: game[turn+2],
    responseType: 'json'
  });
})();
