import pandas as pd
import pyarrow as pa
import pyarrow.feather as feather
import numpy as np

# Step 1: Generate a random pandas DataFrame
def generate_random_dataframe(num_rows: int, num_columns: int):
    data = {f"col_{i}": np.random.rand(num_rows) for i in range(num_columns)}
    df = pd.DataFrame(data)
    return df

# Step 2: Convert pandas DataFrame to PyArrow Table
df = generate_random_dataframe(num_rows=100, num_columns=5)
table = pa.Table.from_pandas(df)

# Step 3: Write the table to a ZSTD compressed Arrow file
output_file = 'input.arrow'

feather.write_feather(table, output_file, compression='zstd')


print(f"Random dataframe written to {output_file} with ZSTD compression.")
