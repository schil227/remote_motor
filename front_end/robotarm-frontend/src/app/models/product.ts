export class Product {
    id: number;
    name: string;
    price: number;
    description: string;

    constructor(id: number, name: string, price: number, description: string){
        this.id = id;
        this.name = name;
        this.price = price;
        this.description = description;
    }
}

export const defaultProduct : Product = {
    id: 0,
    name: '',
    price: 0,
    description: ''
  }