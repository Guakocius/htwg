export default function Game() {
  return (
    <>
      <header id="game-points">0</header>
      <main>
        <div id="player-container">
          <img src="assets/img/flappybird.png" />
        </div>
        <div id="pipe-container">
          {[...Array(6)].map((_, i) => (
            <img
              key={i}
              src="assets/img/pipes.png"
              width={50}
              height={50}
              alt="pipe"
            />
          ))}
        </div>
      </main>
    </>
  );
}
