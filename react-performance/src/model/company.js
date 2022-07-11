import { faker } from '@faker-js/faker';

export default class Company {
    name;
    catchPhrase;
    industry;
    phoneNumber;
    email;
    ipv4;

    constructor(name, catchPhrase, industry, phoneNumber, email, ipv4) {
        this.name = name;
        this.catchPhrase = catchPhrase;
        this.industry = industry;
        this.phoneNumber = phoneNumber;
        this.email = email;
        this.ipv4 = ipv4;
    }

    static generateFake() {
        const name = faker.company.companyName();
        const catchPhrase = faker.company.catchPhrase();
        const industry = faker.company.bsNoun();
        const phoneNumber = faker.phone.number();
        const email = faker.internet.email();
        const ipv4 = faker.internet.ip();
        return new Company(name, catchPhrase, industry, phoneNumber, email, ipv4);
    }

    /// Returns true if the text is contained in the object.
    /// The text is searched in the object's name, description, etc.
    /// The search is case insensitive.
    contains(text) {
        return this.name.toLowerCase().includes(text) ||
            this.catchPhrase.toLowerCase().includes(text) ||
            this.industry.toLowerCase().includes(text) ||
            this.phoneNumber.toLowerCase().includes(text) ||
            this.email.toLowerCase().includes(text) ||
            this.ipv4.toLowerCase().includes(text);
    }
}