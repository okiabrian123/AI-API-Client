document.getElementById('promptForm').addEventListener('submit', async function (e) {
    e.preventDefault();

    const prompt = document.getElementById('prompt').value;
    const option = document.getElementById('model').value;
    const resultContainer = document.getElementById('output');
    
    resultContainer.innerText = 'Generating...';
    let content={
        prompt: prompt,
        option: option, // Make sure this matches your struct
    }
    try {
        const response = await fetch('/api/generate', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(content),
        });

        if (!response.ok) {
            throw new Error('Failed to generate AI response.');
        }

        const data = await response.json();
        resultContainer.innerText = data.response || 'No result returned.';
    } catch (error) {
        resultContainer.innerText = 'Error: ' + error.message;
    }
});