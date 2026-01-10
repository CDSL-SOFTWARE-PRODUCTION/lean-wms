import { StyleSheet, Text, View } from 'react-native';
import { useEffect, useState } from 'react';
import { rspc } from './src/services/api';

export default function App() {
  const [status, setStatus] = useState<string>('Connecting to Backend...');

  useEffect(() => {
    console.log('🚀 Starting connection check...');
    setStatus('Connecting...');

    rspc
      .query(['version'])
      .then((version) => {
        console.log('✅ Connection success! Version:', version);
        setStatus(`✅ Connected! API v${version}`);
      })
      .catch((err) => {
        console.error('❌ Connection failed:', err);
        setStatus(`❌ Error: ${err.message || 'Unknown error'}`);
      });
  }, []);

  return (
    <View style={styles.container}>
      <Text style={styles.text}>Lean WMS Mobile</Text>
      <Text style={styles.status}>{status}</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
  },
  text: {
    fontSize: 24,
    fontWeight: 'bold',
    marginBottom: 20,
  },
  status: {
    fontSize: 16,
    color: '#666',
  },
});
