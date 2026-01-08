"""Loads and analyzes the knowledge graph from CSV files.

To run this script, ensure you have the `grape` library installed,
which can be done via pip, along with `markdownify` for report conversion:

`pip install grape markdownify`.

"""

# pylint: disable=no-name-in-module
from grape import Graph

# pylint: disable=no-name-in-module,import-error
from grape.embedders import Node2VecSkipGramEnsmallen
from markdownify import markdownify
import numpy as np


def main():
    """Loads and analyzes the knowledge graph from CSV files."""
    # Load the knowledge graph from CSV files
    kg = Graph.from_csv(
        directed=False,
        node_type_path="kg_data/directus/node_classes.csv",
        node_types_column="node_class",
        node_path="kg_data/directus/nodes.csv",
        nodes_column="node",
        node_types_separator="|",
        node_list_node_types_column="node_class_ids",
        node_list_numeric_node_type_ids=True,
        edge_type_path="kg_data/directus/edge_classes.csv",
        edge_path="kg_data/directus/edges.csv",
        sources_column="src_id",
        destinations_column="dst_id",
        edge_list_edge_types_column="edge_class_id",
        edge_list_numeric_node_ids=True,
        edge_list_numeric_edge_type_ids=True,
        name="EMI/Directus KG",
        number_of_nodes=12390676,
        number_of_node_types=56,
        number_of_edge_types=130,
    )

    # Filter anything which is not in the main component, which
    # in the case of the Directus KG is removing a vast amount
    # of tuple nodes (i.e. couple of nodes connected by a single edge and
    # not connected to anything else).
    kg_main_component = kg.remove_components(top_k_components=1)

    # Generate a report of the knowledge graph
    report = str(kg_main_component)
    # We convert the report from `HTML` to `Markdown` for better readability
    report = markdownify(report)
    # And save it to a file
    with open("kg_report.md", "w", encoding="utf-8") as report_file:
        report_file.write(report)

    # Generate node embeddings using Node2Vec with Skip-Gram
    embedding = Node2VecSkipGramEnsmallen(verbose=True).fit_transform(kg_main_component)
    node_df, context_df = embedding.get_all_node_embedding()
    # Save the node embeddings to a compressed numpy file
    np.savez_compressed("kg_node_embeddings.npz", node_df.to_numpy())
    np.savez_compressed("kg_context_embeddings.npz", context_df.to_numpy())


if __name__ == "__main__":
    main()
