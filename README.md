## Goal

Input - documentation website (any documentation - pick PyTorch documentation website)
Output - A simple ChatBot server (try to build a UI out of iced-rs) that is able to infer based on the questions, and give meaningful results

## Method:

1. Be able to scrap the data from the docs website
2. Finetune a suitable model -> we'll just do it for minimum number of epochs
    - Be able to figure out what model to use for fine-tuning
    - What are the parameters for you to decide which model to use for fine-tuning?
3. Inference:
    - How would you scale the inference model? --- Number of concurrent calls
    - Performance!

- Fine-tuning we'll try to use simple plain PyTorch (python) for that, but also try torch-rs (rust bindings for PyTorch as an experimental thing)
- Definitely try inference from torch-rs / libtorch

Backup plan:

- Python server - Flask server that is called for a given question and the output is given to the user
