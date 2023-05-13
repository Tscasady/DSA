package main

import (
	"fmt"
	"linked_list/search"
)

// Remove the (first occurance	all occurances) of an element by data content
// Remove an element by position
// Add an element at an arbitrary position
// Add an element after a known node
// Find the distance between two nodes
// type Stringable interface {
//   String() string
// }
// type Addable interface {
//   ~string | ~int
// }

type List[T comparable] struct {
  head *Node[T]
}

type Node[T comparable] struct {
  value T 
  next *Node[T]
}

func (l *List[T]) append(v T) {
 //creates a new node with the provided value 
 // don't do below, it will be created on the stack and deallocated later, 
 // use new or a pointer with & 
 // n := Node {
 //   value: v,
 //   next: nil,
 // }

 n := &Node[T] {
   value: v,
   next: nil,
 }

 tail := l.tail()
 if tail != nil {
   tail.next = n
 } else {
   l.head = n
 }
}

func (l *List[T]) prepend(v T) {
  n := &Node[T] {
    value: v,
    next: l.head,
  }
  l.head = n
}

func (l *List[T]) insert(index int, v T) {
  n:= &Node[T] {
    value: v,
    next: nil,
  }

  if index == 0 {
    l.prepend(v)
  } else {
    current_index := 0
    current_node := l.head
    
    for current_index != index - 1 {
      current_index += 1
      current_node = current_node.next
    } 

    insert_next := current_node.next
    current_node.next = n
    n.next = insert_next
  }

}

func (l *List[T]) pop() Node[T] {
  current_node := l.head
  for current_node.next.next != nil {
    current_node = current_node.next
  }
  popped_node := current_node.next
  current_node.next = nil
  return *popped_node
}

func (l *List[T]) find(index int, number_of_elements int) []T {
  var result []T

  current_index := 0
  current_node := l.head
    
  for current_index != index {
    current_index += 1
    current_node = current_node.next
  } 

  for i := 0; i < number_of_elements; i++ {
    result = append(result, current_node.value)
    current_node = current_node.next
    if current_node == nil {
      return result
    }
  }
  return result
}

func (l *List[T]) includes(v T) bool {
  current_node := l.head
  for current_node != nil {
    if v == current_node.value {
      return true
    } else {
      current_node = current_node.next
    }
  }
  return false
}

// have to pass a reference, pass by value by default, so you won't change original if you pass l List
func (l *List[T]) tail() *Node[T] {
  current_node := l.head
  if current_node != nil {
    for current_node.next != nil {
      current_node = current_node.next
    }
  } 
  
  return current_node
}

func (l *List[T]) count() int {
  current_node := l.head
  count := 0
  for current_node != nil {
    count += 1
    current_node = current_node.next
  }
  return count
}

// func (l *List[T]) to_string() string {
//   current_node := l.head
//   var value string
//   for current_node != nil {
//     if current_node.next != nil {
//       value += current_node.value.String()+ " ->"
//     } else {
//       value += current_node.value.String()
//     }
//     current_node = current_node.next
//   }
//   return value
// }

// func (l *List[T]) to_string() string {
//   current_node := l.head
//   var value string
//   for current_node != nil {
//     if current_node.next != nil {
//       value += current_node.value + " -> "
//     } else {
//       value += current_node.value
//     }
//     current_node = current_node.next
//   }
//   return value
// }

// func traverse(l *List) 


func main() {
  l := List[string]{ head: nil }
  l.append("ploop")  
  l.append("plop")  
  fmt.Println(l.head)
  l.prepend("dog")
  l.insert(2, "hi?")
  // fmt.Println(l.to_string())
  fmt.Println(l.find(1, 2))
  fmt.Println(l.includes("not"))
  fmt.Println(l.includes("hi?"))
  fmt.Println(l.pop())
  // fmt.Println(l.to_string())
  fmt.Println(l.count())
  // fmt.Println(l.to_string())
  var array []int = []int{1, 2, 3, 4, 5} 
  fmt.Println(search.Binary_search(2, array))
}

