const got = require('got');

const file = process.argv[2]
const turn = Number(process.argv[3]);

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
  }
})();
