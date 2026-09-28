const output = document.querySelector('#output');
const ping = document.querySelector('#ping');

ping.addEventListener('click', async () => {
  ping.disabled = true;
  try {
    const response = await fetch('/invoke', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ action: 'ping', payload: '' }),
    });
    output.textContent = JSON.stringify(await response.json(), null, 2);
  } catch (error) {
    output.textContent = String(error);
  } finally {
    ping.disabled = false;
  }
});
