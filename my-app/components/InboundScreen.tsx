import React, { useState } from 'react';
import { View, Text, TextInput, Button, FlatList, StyleSheet, Alert } from 'react-native';
import { observer } from '@legendapp/state/react';
import { store, addProduct } from '../lib/store';

const InboundScreen = observer(() => {
    const [barcode, setBarcode] = useState('');
    const [name, setName] = useState('');
    const [sku, setSku] = useState('');

    const products = store.products.get();
    const productsList = Object.values(products || {}).filter(p => !p.deleted);

    const handleAddProduct = () => {
        if (!barcode || !sku || !name) {
            Alert.alert('Error', 'Please fill all fields');
            return;
        }

        // Simulating a scan and check
        // In real app, we check if SKU exists, etc.

        try {
            addProduct(sku, name, barcode);
            Alert.alert('Success', 'Product added locally and syncing...');
            setBarcode('');
            setName('');
            setSku('');
        } catch (e: any) {
            Alert.alert('Error', e.message);
        }
    };

    return (
        <View style={styles.container}>
            <Text style={styles.title}>Inbound: Scan & Add</Text>

            <View style={styles.form}>
                <TextInput
                    style={styles.input}
                    placeholder="Scan Barcode"
                    value={barcode}
                    onChangeText={setBarcode}
                />
                <TextInput
                    style={styles.input}
                    placeholder="SKU Code"
                    value={sku}
                    onChangeText={setSku}
                />
                <TextInput
                    style={styles.input}
                    placeholder="Product Name"
                    value={name}
                    onChangeText={setName}
                />
                <Button title="Save Product" onPress={handleAddProduct} />
            </View>

            <Text style={styles.subtitle}>Recent Products (Synced)</Text>
            <FlatList
                data={productsList}
                keyExtractor={(item) => item.id}
                renderItem={({ item }) => (
                    <View style={styles.item}>
                        <Text style={styles.itemText}>{item.name} ({item.sku})</Text>
                        <Text style={styles.itemSub}>{item.barcode}</Text>
                        <Text style={styles.itemSub}>{item.id}</Text>
                    </View>
                )}
            />
        </View>
    );
});

const styles = StyleSheet.create({
    container: {
        flex: 1,
        padding: 20,
        backgroundColor: '#f5f5f5',
    },
    title: {
        fontSize: 24,
        fontWeight: 'bold',
        marginBottom: 20,
    },
    subtitle: {
        fontSize: 18,
        fontWeight: '600',
        marginTop: 20,
        marginBottom: 10,
    },
    form: {
        backgroundColor: 'white',
        padding: 15,
        borderRadius: 10,
        gap: 10,
    },
    input: {
        borderWidth: 1,
        borderColor: '#ddd',
        padding: 10,
        borderRadius: 5,
        marginBottom: 10,
    },
    item: {
        backgroundColor: 'white',
        padding: 15,
        borderRadius: 8,
        marginBottom: 8,
    },
    itemText: {
        fontWeight: 'bold',
    },
    itemSub: {
        color: '#666',
        fontSize: 12,
    },
});

export default InboundScreen;
