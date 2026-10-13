// Quick-create modal for adding a new item from the invoice pages.
// Expects #quick-item-modal, #qi-name, #qi-rate, #qi-error and #item-select.

function closeQuickItem() {
    document.getElementById('quick-item-modal').classList.remove('open');
    document.getElementById('qi-name').value = '';
    document.getElementById('qi-rate').value = '';
    const err = document.getElementById('qi-error');
    err.style.display = 'none';
    err.textContent = '';
}

async function quickCreateItem() {
    const name = document.getElementById('qi-name').value.trim();
    const rate = document.getElementById('qi-rate').value.trim();
    const err  = document.getElementById('qi-error');

    const fail = (msg) => {
        err.textContent = msg;
        err.style.display = 'block';
    };

    if (!name || !rate) return fail('Name and rate are required.');

    let item;
    try {
        const res = await fetch('/items/quick', { method: 'POST', body: new URLSearchParams({ name, rate }) });
        if (!res.ok) return fail(await res.text() || 'Failed to create item.');
        item = await res.json();
    } catch (e) {
        return fail('Failed to create item.');
    }

    const sel = document.getElementById('item-select');
    const opt = document.createElement('option');
    opt.value        = item.id;
    opt.dataset.name = item.name;
    opt.dataset.rate = item.rate;
    opt.textContent  = `${item.name} ($${item.rate})`;
    sel.appendChild(opt);
    sel.value = String(item.id);

    closeQuickItem();
}
