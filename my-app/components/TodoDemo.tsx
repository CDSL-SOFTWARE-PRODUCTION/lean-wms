import { observer } from '@legendapp/state/react'
import { addTodo, todos$ as _todos$, toggleDone } from '../lib/supalegend'
import { useState } from 'react'
import { FlatList, NativeSyntheticEvent, StyleSheet, Text, TextInput, TouchableOpacity, View } from 'react-native'
import { Tables } from '../lib/database.type'

// Emojis to decorate each todo.
const NOT_DONE_ICON = String.fromCodePoint(0x1f7e0)
const DONE_ICON = String.fromCodePoint(0x2705)

const Todos = observer(({ todos$ }: { todos$: typeof _todos$ }) => {
    // Get the todos from the state and subscribe to updates
    const todos = todos$.get()

    const renderItem = ({ item: todo }: { item: Tables<'todos'> }) => <Todo todo={todo} />

    if (todos) {
        return (
            <FlatList
                data={Object.values(todos)}
                renderItem={renderItem}
                style={styles.todos}
                keyExtractor={(item) => item.id}
            />
        )
    }

    return <></>
})

// The text input component to add a new todo.
const NewTodo = () => {
    const [text, setText] = useState('')
    const handleSubmitEditing = ({ nativeEvent: { text } }: NativeSyntheticEvent<{ text: string }>) => {
        setText('')
        addTodo(text)
    }
    return (
        <TextInput
            value={text}
            onChangeText={(text) => setText(text)}
            onSubmitEditing={handleSubmitEditing}
            placeholder="What do you want to do today?"
            style={styles.input}
        />
    )
}

// A single todo component, either 'not done' or 'done': press to toggle.
const Todo = ({ todo }: { todo: Tables<'todos'> }) => {
    const handlePress = () => {
        toggleDone(todo.id)
    }
    return (
        <TouchableOpacity
            key={todo.id}
            onPress={handlePress}
            style={[styles.todo, todo.done ? styles.done : null]}
        >
            <Text style={styles.todoText}>
                {todo.done ? DONE_ICON : NOT_DONE_ICON} {todo.text}
            </Text>
        </TouchableOpacity>
    )
}

export default function TodoDemo() {
    return (
        <View style={styles.container}>
            <NewTodo />
            <Todos todos$={_todos$} />
        </View>
    )
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        padding: 24,
        backgroundColor: '#fff',
    },
    todos: {
        marginTop: 16,
        flex: 1,
    },
    input: {
        height: 48,
        borderWidth: 1,
        borderColor: '#ddd',
        borderRadius: 8,
        paddingHorizontal: 16,
        fontSize: 16,
        backgroundColor: '#f9f9f9',
    },
    todo: {
        flexDirection: 'row',
        alignItems: 'center',
        padding: 16,
        borderBottomWidth: 1,
        borderBottomColor: '#eee',
    },
    done: {
        opacity: 0.5,
    },
    todoText: {
        fontSize: 16,
    },
})
