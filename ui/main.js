const output = document.querySelector('#output');
const ping = document.querySelector('#ping');

async function invoke(action, payload = '') {
  const response = await fetch('/invoke', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ action, payload }),
  });
  return response.json();
}

ping.addEventListener('click', async () => {
  ping.disabled = true;
  try {
    output.textContent = JSON.stringify(await invoke('ping'), null, 2);
  } catch (error) {
    output.textContent = String(error);
  } finally {
    ping.disabled = false;
  }
});
